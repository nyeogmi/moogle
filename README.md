# Moogle

`moogle` is a relational data store in Rust. Specifically, it does junction tables. 

In less jargon-y words: it's an alternative to `HashMap` with a lot of desirable properties for games and simulations. It only operates on `Copy` values because it's designed to be used with a table system that assigns IDs or UUIDs.

I haven't profiled it yet, but based on its implementation (a very thin layer over `BTreeMap`) I'd expect it to be within an order of magnitude of `BTreeMap` in all respects. 

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
println!("Jochen's items: {:?}", inventory.get(russell));  
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

`moogle` solves this problem by tracking the answer to "who owned the stick previously?" using a data structure called a bimap, then using SQL-style constraints to make sure only one person owns it:

```rust
let inventory: OneToSet<NPC, Item> = OneToSet::new();
inventory.mut_fwd().insert(russell, stick);
inventory.mut_fwd().insert(russell, beetle);
inventory.mut_fwd().insert(russell, pokemon_card);
inventory.mut_fwd().insert(jochen, pizza);

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

If you're a Rust or Python guy, you can think of it as two dictionaries that are kept in sync. For instance, the `SetToOne` used above is exactly equivalent to a `BTreeMap<NPC, BTreeSet<Item>>` paired with a `BTreeMap<Item, NPC>`.

(What does "kept in sync" mean? Formally, it means that for every `(NPC, Item)` stored in the first one, there's a corresponding `(Item, NPC)` in the second, and vice versa.)

It also solves several other problems:

- the order of the items in a Moogle store is deterministic (based on `Ord`)
- the data type of the result you get reflects whether a `UNIQUE` constraint could have existed
- calling `.get_mut()` on a map resulting in a Set gets you a Set which automatically updates the opposed dictionary when you alter it.

In case the last property needs some some demonstration:

```rust
    let inventory: OneToSet<NPC, Item> = OneToSet::new();
    inventory.mut_fwd().insert(russell, stick);
    inventory.mut_fwd().insert(russell, beetle);
    inventory.mut_fwd().insert(russell, pokemon_card);
    inventory.mut_fwd().insert(jochen, pizza);

    // this is exactly equivalent to calling inventory.mut_fwd().insert(jochen, stick)
    let jochens_items = inventory.mut_fwd().get_mut(jochen);
    jochens_items.insert(stick);
```

The UNIQUE constraints Moogle provides are completely optional; the SetToSet type has none, meaning that the answer to any `get()` operation is a set instead of a single result:

```rust
    let visits: SetToSet<NPC, Place> = SetToSet::new();
    visits.mut_fwd().insert(marcia, paris);
    visits.mut_fwd().insert(marcia, rome);
    visits.mut_fwd().insert(gavin, rome);
    visits.mut_fwd().insert(gavin, london);
    visits.mut_fwd().insert(smith, london);
    visits.mut_fwd().insert(smith, venice);

    println!("Who visits London? {:?}", visits.bwd().get(london));
        // => smith, gavin
    println!("Where does Gavin visit? {:?}", visits.fwd().get(gavin));
        // => rome, london

    // modify set without using key
    visits.mut_fwd().get_mut(gavin).insert(texas);
    println!("Where does Gavin visit? {:?}", visits.fwd().get(gavin));
        // => rome, london, texas
```



## Formally

`moogle` provides an internal bimap type with four public specializations. Each is equivalent to a different kind of junction table.

The four specializations are below:

- `OneToOne`: maps `Optional<T>` to `Optional<T>`
- `OneToSet`: maps `Optional<T>` to `BTreeSet<T>`
- `SetToOne`: maps `BTreeSet<T>` to `Optional<T>`
- `SetToSet`: maps `BTreeSet<T>` to `BTreeSet<T>`

The bimap is based on `BTreeMap`, meaning it preserves `Ord` of elements and is deterministic. Elements are required to be `PartialEq`, `Ord` and `Copy`. (Some examples of types satisfying these requirements are numeric IDs and UUIDs.)

Each specialization can be viewed in a forwards direction (using the `.fwd()` accessor) and a backwards direction (using the `.bwd()` accessor) -- for instance, `OneToSet<usize, char>` corresponds to a `BTreeMap<usize, BTreeSet<char>>` and a `BTreeMap<char, usize>` that are always kept in sync. 

(What does "kept in sync" mean? Formally: `insert()`ing on one automatically `insert()`s on the other such that each pair `(a, b)` in one has a corresponding pair `(b, a)` in the other.)

`Set`-based bimaps provide an `Entry`-like interface for insertions, as well as a `.items()` iterator that averts that interface.