# Moogle

`moogle` provides a bimap type with four specializations. This data structure can stand in for a hashtable in programs where data is represented relationally. Specifically, it is equivalent to a junction table.

The four specializations are below:

- `OneToOne`: maps `Optional<T>` to `Optional<T>`
- `OneToSet`: maps `Optional<T>` to `BTreeSet<T>`
- `SetToOne`: maps `BTreeSet<T>` to `Optional<T>`
- `SetToSet`: maps `BTreeSet<T>` to `BTreeSet<T>`

The bimap is based on `BTreeMap`, meaning it preserves `Ord` of elements. Elements are required to be `PartialEq`, `Ord` and `Copy`. (Some examples of types satisfying these requirements are numeric IDs and UUIDs.)

Each specialization can be viewed in a forwards direction (using the `.fwd()` accessor) and a backwards direction (using the `.bwd()` accessor) -- for instance, `OneToSet<usize, char>` corresponds roughly to a `BTreeMap<usize, BTreeSet<char>>` and a `BTreeMap<char, usize>` that are always kept in sync. 

(That is, `insert()`ing on one automatically `insert()`s on the other, and the pairs `(a, b)` in each are equal to the pairs `(b, a)` in the other.)

Sets and optionals are disjoint: an element that appears in one set on the left side can't appear in any other set on the left side, and the same holds for the right side.

`Set`-based specializations provide an `Entry`-like interface for insertions, as well as a `.items()` iterator that averts that interface.