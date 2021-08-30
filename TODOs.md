
## TODOs

Small:

- Make documentation reflect that data structures and poms are available
- Relax the lifetime constraints involving raw junctions. (Should be possible now that the vset implementation is changed.)
- Better `len()` implementation for ToSet
- Proptest lexicographic order

Medium:

- Support miri
- IntoIterator for &T for all builtin `.iter()`-providing Ts.
- Add `arr[x]` syntax for all builtin junctions.
- The other traits et cetera from BTreeMap and BTreeSet

Big:

- More thorough unit testing
- Document everything

Speculative:

- Allow multiple junctions to share a database and allow Prolog-like querying across that database.
- Add ToVecs, which behave like ToSets except:
  - tracks and evicts based on insertion order
  - duplicate items are allowed
  - can be capped
  - Consider: ToVec-based junctions are no longer unique in what pairs they contain?
  - Consider: ToOrderedSet instead, or ToIxd? (indexed, but keeping the pairwise uniqueness guarantee)

Misc:

- Cute moogle logo.