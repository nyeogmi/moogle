# Moogle

`moogle` is an alternative to `BTreeMap` with a lot of desirable properties for game-like simulations. These include determinism, symmetry, and uniqueness of associations between keys/values.

Its interface is fairly close to Rust's builtin map interface. I haven't profiled it yet, but based on its implementation, I'd expect it to be within an order of magnitude of BTreeMap in all respects. 

Specifically: `moogle` provides an internal bimap type with four public specializations. Each is equivalent to a different kind of junction table.

The four specializations are below:

- `OneToOne`: maps `Optional<T>` to `Optional<T>`
- `OneToSet`: maps `Optional<T>` to `BTreeSet<T>`
- `SetToOne`: maps `BTreeSet<T>` to `Optional<T>`
- `SetToSet`: maps `BTreeSet<T>` to `BTreeSet<T>`

The bimap is based on `BTreeMap`, meaning it preserves `Ord` of elements and is deterministic. Elements are required to be `PartialEq`, `Ord` and `Copy`. (Some examples of types satisfying these requirements are numeric IDs and UUIDs.)

Each specialization can be viewed in a forwards direction (using the `.fwd()` accessor) and a backwards direction (using the `.bwd()` accessor) -- for instance, `OneToSet<usize, char>` corresponds roughly to a `BTreeMap<usize, BTreeSet<char>>` and a `BTreeMap<char, usize>` that are always kept in sync. 

(That is, `insert()`ing on one automatically `insert()`s on the other, and the pairs `(a, b)` in each are equal to the pairs `(b, a)` in the other.)

Sets and optionals are disjoint: an element that appears in one set on the left side can't appear in any other set on the left side, and the same holds for the right side.

`Set`-based specializations provide an `Entry`-like interface for insertions, as well as a `.items()` iterator that averts that interface.