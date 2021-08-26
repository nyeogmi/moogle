# Moogle

`moogle` is a collection of types for representing data relationally in Rust. The library assumes you're using a slot map implementation of some kind to assign IDs to the values in your program -- it then provides mapping types called junctions that allow you to track the relationships from IDs to each other. 

`moogle` provides data structures for the four two-way relationship types used in database programming: one-to-one, one-to-many, many-to-one, and many-to-many. For each type, it provides two data structures:

- a "raw" data structure (ex. RawOneToOne): a very thin implementation of that relationship over `BTreeMap`
- a "shared" data structure (ex. OneToOne): a shareable implementation of that relationship over `RefCell<BTreeMap>`

I haven't profiled the Moogle types yet, but based on their implementation (a very thin layer over `BTreeMap`) I'd expect the raw types to be within an order of magnitude of `BTreeMap` in all cases. In cases with a lot of reads and few writes, I would expect the shared versions to approach `BTreeMap` performance but not reach it.

## Motivation

Let's say you're building a game where NPCs collect items and you want to track what items are in each NPC's inventory:

```rust
let mut inventory: HashMap<NPC, Vec<Item>> = BTreeMap::new();
```

This representation has the following problems:

- if an NPC has no items, you have to manually initialize the Vec
- if an NPC runs out of items, you have to manually remove the Vec

Many people deal with these two problems by creating a MultiMap type:

```rust
let mut inventory: MultiMap<NPC, Item> = MultiMap::new();
```

Some example code in this system might look like this:

```rust
inventory.insert(russell, stick);
inventory.insert(russell, beetle);
inventory.insert(russell, pokemon_card);
inventory.insert(jochen, pizza);

println!("Russell's items: {:?}", inventory.get(russell)); 
    // => stick, beetle, pokemon_card
println!("Jochen's items: {:?}", inventory.get(jochen));  
    // => pizza
```

However, there are some possible problems with this system.

For one thing, a naive implementation of multimaps might fail to take an item out of Russell's inventory once Jochen gets it:

```rust
inventory.insert(jochen, stick);

println!("Russell's items: {:?}", inventory.get(russell)); 
    // => stick, beetle, pokemon_card
println!("Jochen's items: {:?}", inventory.get(jochen));  
    // => pizza, stick
```

This is not ideal. You can deal with this by tracking whose inventory the stick was taken from before calling `inventory.insert()`, but it would be better not to have to keep track of conservation of mass by changing your game logic.

`moogle` solves this problem by tracking the answer to "who owned the stick previously?" using a data structure called a junction, then using SQL-style constraints to make sure only one person owns it:

```rust
let inventory: OneToSet<NPC, Item> = OneToSet::new();
inventory.fwd().insert(russell, stick);
inventory.fwd().insert(russell, beetle);
inventory.fwd().insert(russell, pokemon_card);
inventory.fwd().insert(jochen, pizza);

println!("Russell's items: {:?}", inventory.fwd().get(russell)); 
    // => stick, beetle, pokemon_card
println!("Jochen's items: {:?}", inventory.fwd().get(russell));  
    // => pizza

println!("Who owns the stick? {:?}", inventory.bwd().get(stick));
    // => russell

inventory.mut_fwd().insert(jochen, stick);

println!("Russell's items: {:?}", inventory.fwd().get(russell)); 
    // => beetle, pokemon_card
println!("Jochen's items: {:?}", inventory.fwd().get(jochen));  
    // => pizza, stick
```

If you're a relational database guy, you can think of a Moogle bimap as a junction table with two columns where either column can be `UNIQUE`, and constraint violations are dealt with by deleting the older row.

If you're a Rust or Python guy, you can think of it as two dictionaries that are kept in sync. For instance, the `RawSetToOne` used above is exactly equivalent to a `BTreeMap<NPC, BTreeSet<Item>>` paired with a `BTreeMap<Item, NPC>`.

(What does "kept in sync" mean? Formally, it means that for every `(NPC, Item)` stored in the first one, there's a corresponding `(Item, NPC)` in the second, and vice versa.)

`moogle` also solves several other problems:

- the order of the items in a Moogle store is deterministic (based on `Ord`)
- the data type of the result you get reflects whether a `UNIQUE` constraint could have existed

`moogle` also provides some shorthand for code that would like to pretend that sets inside the map are actually owned by the objects that use them. The shorthand is patterned after the Entry API for existing maps, and looks like this:

```rust
    let inventory: OneToSet<NPC, Item> = OneToSet::new();
    inventory.fwd().insert(russell, stick);
    inventory.fwd().insert(russell, beetle);
    inventory.fwd().insert(russell, pokemon_card);
    inventory.fwd().insert(jochen, pizza);

    // shorthand: this is exactly equivalent to calling inventory.fwd().insert(jochen, stick)
    let jochens_items = inventory.fwd().get(jochen);
    jochens_items.insert(stick);
```

The UNIQUE constraints Moogle provides are completely optional; the SetToSet type has none, meaning that the answer to any `get()` operation is a set instead of a single result:

```rust
    let visits: SetToSet<NPC, Place> = SetToSet::new();
    visits.fwd().insert(marcia, paris);
    visits.fwd().insert(marcia, rome);
    visits.fwd().insert(gavin, rome);
    visits.fwd().insert(gavin, london);
    visits.fwd().insert(smith, london);
    visits.fwd().insert(smith, venice);

    println!("Who visits London? {:?}", visits.bwd().get(london));
        // => smith, gavin
    println!("Where does Gavin visit? {:?}", visits.fwd().get(gavin));
        // => rome, london

    // modify set without using key
    visits.fwd().get(gavin).insert(texas);
    println!("Where does Gavin visit? {:?}", visits.fwd().get(gavin));
        // => rome, london, texas
```

## Sharing

Many programming languages allow programmers to write code that modifies a data structure at the same time as it iterates over it. 

For instance, try entering this into Python 3:

```python
>>> xs = {1: "a", 2: "b"}
>>> for x, y in xs.items():
...   xs[3] = "c"
...   print(x, y)
...
1 a
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
RuntimeError: dictionary changed size during iteration
```

The situation is more tense in Rust, where holding a reference to an entry inside a data structure or an iterator over that data structure prevents mutation of the whole data structure, even if that mutation would not change the data structure's shape. That means that this benign Python program is not allowed to be written in Rust:

```python
>>> xs = {1: "scarf", 2: "ghost", "beware the": []}
>>> for x, y in xs.items():
...   if x in [1, 2]:
...     xs["beware the"].append(y)
...   else:
...     print(x, y)
...
beware the ['scarf', 'ghost']
```

(There are numerous other reasons why this program is impossible to write in Rust, most involving the type system, but the `.append()` in particular is a direct violation of the borrow made by items().)

In `moogle`, all of these operations are gated by `RefCell` for safety and given predictable behavior. In particular:

- If an iterator has lexicographically passed an element, then it can't observe changes to that element. Otherwise, it can.
- The only internal data structure you're allowed to hold a pointer to is a `FwdSet`/`BwdSet`, and it sees all changes made to the underlying store as soon as they happen.

This behavior is basically the same as the behavior of Redis `ZSCAN`. If this behavior spooks you out, you can use `RawSetToSet` etc instead of `SetToSet` etc, which will make it impossible for it to affect you, and which will probably offer you a performance boost too. 

The implementation is pretty sane but makes use of `unsafe` in about two places. (see the `Formally` section for details) Unsafe code is fuzzed for safety.

For a quick demo of the lexicographic passage rule, see the below:

```rust
    let letters: OneToSet<Alphabet, char> = SetToOne::new();
    letters.fwd().insert(english, 'a');
    letters.fwd().insert(english, 'b');
    letters.fwd().insert(english, 'd');
    letters.fwd().insert(english, 'e');

    let asc = letters.fwd().items();

    asc.next();  // (english, 'a')
    asc.next();  // (english, 'b')
    asc.next();  // (english, 'd')
    letters.fwd().insert(english, 'c')

    // don't see the 'c', we already passed it
    asc.next();  // (english, 'e')
    asc.next();  // None
```

This works for reversed iterators too -- in that case, if we passed it in reverse, we don't see it.

For a quick demo of the held set rule:

```rust
    let possessions: SetToSet<Ghost, Item> = SetToSet::new();
    let sylvian_possessions = possessions.fwd().get(sylvian);
    println!("{:?}", sylvian_possessions.get());  // nothing
    possessions.fwd().insert(sylvian, scarf)
    println!("{:?}", sylvian_possessions.get());  // {scarf}
```

These two rules play along without surprises:

```rust
    let letters: OneToSet<Alphabet, char> = SetToOne::new();
    let english_letters = letters.fwd().get(english);
    letters.fwd().insert(english, 'a');
    letters.fwd().insert(english, 'b'):
    letters.fwd().insert(english, 'c');
    letters.fwd().insert(english, 'd');

    let desc = english_letters.items().rev();

    desc.next();  // 'd'
    desc.next();  // 'c'
    desc.next();  // 'b'
    letters.fwd().insert(english, 'f')

    // don't see the 'f', we already passed it
    desc.next();  // 'a'
    desc.next();  // None
```


## Formally

`moogle` provides two sets of junctions -- the `raw` set and the `shared` set. 

### The raw set

The raw set is an internal bimap type with four public specializations. Each is equivalent to a different kind of relation.

The four specializations are below:

- `RawOneToOne<A, B>`: maps `Optional<A>` to `Optional<B>`
- `RawOneToSet<A, B>`: maps `Optional<A>` to `BTreeSet<B>`
- `RawSetToOne<A, B>`: maps `BTreeSet<A>` to `Optional<B>`
- `RawSetToSet<A, B>`: maps `BTreeSet<A>` to `BTreeSet<B>`

The underlying mapping is done using `BTreeMap`, which preserves `Ord` of elements and is deterministic. Elements are required to be `PartialEq`, `Ord` and `Copy`. (Some examples of types satisfying these requirements are numeric IDs and UUIDs.)

Each specialization can be viewed in a forwards direction (using the `.fwd()` accessor) and a backwards direction (using the `.bwd()` accessor) -- for instance, `RawOneToSet<usize, char>` corresponds to a `BTreeMap<usize, BTreeSet<char>>` and a `BTreeMap<char, usize>` that are always kept in sync. 

(What does "kept in sync" mean? Formally: `insert()`ing on one automatically `insert()`s on the other such that each pair `(a, b)` in one has a corresponding pair `(b, a)` in the other.)

`Set`-based junctions provide an `Entry`-like interface for insertions, as well as an iterator that averts that interface.

### The shared set

The shared set consists of four more types. Here's their names and their respective implementations:

- OneToOne<A, B>: `MoogCell<RawOneToOne<A, B>>`
- OneToSet<A, B>: `MoogCell<RawOneToSet<A, B>>`
- SetToOne<A, B>: `MoogCell<RawSetToOne<A, B>>`
- SetToSet<A, B>: `MoogCell<RawSetToSet<A, B>>`

`MoogCell` is a special type implemented using `unsafe` code which allows the user to keep refs to the inside of a `RefCell` by keeping a generation counter on the `RefCell` itself. 

Each time the generation counter is incremented, the refs pointing into the `MoogCell` are invalidated.

(Currently this is safety-tested with QuickCheck. Future safety-testing will involve miri too.)