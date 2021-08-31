# Moogle

`moogle` is a Rust library for relational programming. 

Relational programming is the underlying model used by SQL databases. In the relational model, all data is represented using global sets which support joins. 

Most languages can support this model fairly well because, unlike Rust, they support concurrent writers and iterators, as long as the writers don't change the shape of the underlying data structure. 

The point of `moogle` is to make relational code as easy to write in Rust as it is to write in Python or SQL.

## Motivation

Let's say you're building a game where NPCs collect items and you want to track what items are in each NPC's inventory. 

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

`moogle` solves this problem by tracking the answer to "who owned the stick previously?" then using that answer to clear out the previous owner:

```rust
let inventory: OneToMany<NPC, Item> = OneToMany::new();
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

`moogle` also solves some other problems: all operations on `moogle` data structures are deterministic, and despite the fact that they support sharing, `moogle` data structures do not panic.

## What's in the box?

`moogle` provides eight data structures. Each one represents a different table type that you might expect to see in a relational database, and each provides natural type-safe Rust interface.

The fundamental data structure in `Pom`, a table type. `Pom` is a container whose only purpose is to store things -- adding something to a `Pom` assigns it an `Id` (an opaque integer value) you can use to fetch it again. These integer values satisfy the `IdLike` trait, which most other `moogle` data structures require.

In addition to that, it provides tabular representations of three familiar data structures:

- `ToOne` and `ToMany`: unidirectional map types analogous to `Map<K, V>` and `Map<K, Set<V>>` respectively
- `Set`: a generic sorted set type analogous to `BTreeSet<K>`

It provides four types implemented in terms of these, called junctions, representing relationships between entities. Each is a pair of maps kept in sync:

- `OneToOne<A, B>`: `Map<A, B>` and `Map<B, A>` 
- `OneToMany<A, B>`: `Map<A, Set<B>>` and `Map<B, A>` 
- `ManyToOne<A, B>`: `Map<A, B>` and `Map<B, Set<A>>`
- `ManyToMany<A, B>`: `Map<A, Set<B>>` and `Map<B, Set<A>>`

You can figure out which relationship you need by thinking about how many items from set B each item from set A can be associated with, and vice versa in the other direction.

For instance, here's an example of each relationship, as applied to vampire bats:

- Each bat has one true name. (and no more) Each true name belongs to one bat. (and no more) (`OneToOne<Bat, TrueName>`)
- Each bat has many secrets. Each secret belongs to one bat. (and no more) (`OneToMany<Bat, Secret>`)
- Each bat has one cave. (and no more) Each cave belongs to many bats. (`ManyToOne<Bat, Cave>`)
- Each bat has many victims. Each victim belongs to many bats. (`ManyToMany<Bat, Victim>`)

## What properties do Moogle data structures have?

`moogle` data structures are designed around sacrificing performance to provide a simpler API or achieve greater consistency. Below is a quick summary of the design decisions `moogle` makes:

### API

All `moogle` data structures support the `Set` or `Map` interface. (They are roughly API-compatible with `BTreeSet` and `BTreeMap`, with some extra boilerplate due to their symmetrical structure.)

`moogle` data structures do not panic at runtime.

### Determinism

All `moogle` data structures are sorted. What that means is that they can be iterated in `Ord` order by calling `.iter()`. They can also be iterated in reverse order by calling `.iter().rev()`.

All built-in operations on `moogle` data structures are deterministic. This is a consequence of the fact that they're in sorted order.

Some advice: `moogle`'s `Pom`s sacrifice significant performance to guarantee sorted order and deterministic ID values. If you don't care about that at all, see if you can use the `slot_map` library instead: it's very fast! 

### Concurrency

All `moogle` data structures allow iterators and writers at the same time. For `Pom`, any operation that does not change the number of keys is allowed. For all other data structures, it's impossible to hold a dangling reference to an interior value, and therefore every operation is allowed.

All `moogle` data structures come with a separate `Raw` version -- this version does not support concurrent iterators and writers, but has faster performance. For every type except `Pom`, a `.raw()` accessor exists to temporarily borrow the structure as an instance of the `Raw` type, which will enable you to achieve the same performance for the duration of the borrow.

(Unfortunately, for `Pom`, the raw representation is very different for performance reasons, and no such borrow is possible.)

### Symmetry

`moogle` junctions (`OneToOne`, `OneToMany`, `ManyToOne`, and `ManyToMany`) have the extra property of symmetry. 

For any junction, if `j.fwd().iter()` contains `(a, b)`, then `j.bwd().iter()` contains `(b, a)` (and vice versa)

## Concurrency specifics 

Many programming languages allow programmers to write code that modifies a data structure at the same time as it iterates over it. Usually the defined behavior for this situation is to crash as soon as the situation is noticed.

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
The situation is more tense in Rust, where more things are disallowed. Holding a reference to an entry inside a data structure or an iterator over that data structure prevents mutation of any part of the data structure, even if that mutation would not change the data structure's shape. That means that this benign Python program is not allowed to be written in Rust:

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

(There are numerous other reasons why this program is impossible to write in Rust, most involving the type system, but the `.append()` in particular is a direct violation of the borrow made by `.items()`.)

In `moogle`, all of these operations are gated by `RefCell` for safety, then given predictable behavior. In particular:

- If an iterator has passed an element Ord-wise, then it can't observe changes to that element. Otherwise, it can.
- The only interior data structure you're allowed to hold a pointer to is a `FwdSet`/`BwdSet`, and it sees all changes made to the underlying store as soon as they happen.

This behavior is basically the same as the behavior of Redis `ZSCAN`. If this behavior spooks you out, you can use `RawManyToMany` etc instead of `ManyToMany` etc, which will make it impossible for it to affect you, and which will probably offer you a performance boost too. 

The implementation is pretty sane but makes use of `unsafe` in about two places. (see the `Formally` section for details) Unsafe code is fuzzed for safety.

For a quick demo of the passage rule, see the below:

```rust
    let letters: OneToMany<Alphabet, char> = OneToMany::new();
    letters.fwd().insert(english, 'a');
    letters.fwd().insert(english, 'b');
    letters.fwd().insert(english, 'd');
    letters.fwd().insert(english, 'e');

    let asc = letters.fwd().items();

    asc.next();  // (english, 'a')
    asc.next();  // (english, 'b')
    asc.next();  // (english, 'd')
    letters.fwd().insert(english, 'c');

    // don't see the 'c', we already passed it
    asc.next();  // (english, 'e')
    asc.next();  // None
```

This works for reversed iterators too -- in that case, if we passed it in reverse, we don't see it.

For a quick demo of the held set rule:

```rust
    let possessions: OneToMany<Ghost, Item> = OneToMany::new();
    let sylvian_possessions = possessions.fwd().get(sylvian);
    println!("{:?}", sylvian_possessions.get());  // nothing
    possessions.fwd().insert(sylvian, plush)
    println!("{:?}", sylvian_possessions.get());  // {plush}
    sylvian_possessions.insert(fangs);
    possessions.iter(); // {(sylvian, plush), (sylvian, fangs)}
```

These two rules play along without surprises:

```rust
    let letters: OneToMany<Alphabet, char> = OneToMany::new();
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