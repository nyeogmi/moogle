
## TODOs

Small:

- Unit test NUMBER OF KEYS PER SET (not just "overall count") 
  (I just rewrote this)
- Relax the lifetime constraints involving raw junctions. (Should be possible now that the vset implementation is changed.)
- Proptest lexicographic order

Medium:

- More operations for poms and data structures too
- Don't allocate in the ToMany impl of Serde
- Support miri
- Integration testing
- Add `arr[x]` syntax for all builtin junctions.
- The other traits et cetera from BTreeMap and BTreeSet

Big:

- Write some real examples
- More thorough unit testing
- Automatic benchmarking
- Document everything

Speculative:

- Allow multiple junctions to share a database and allow Prolog-like querying across that database.
- Add ToVecs, which behave like ToManys except:
  - tracks and evicts based on insertion order
  - duplicate items are allowed
  - can be capped
  - Consider: ToVec-based junctions are no longer unique in what pairs they contain?
  - Consider: ToOrderedSet instead, or ToIxd? (indexed, but keeping the pairwise uniqueness guarantee)

Misc:

- Cute moogle logo.