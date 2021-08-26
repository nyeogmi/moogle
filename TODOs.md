
## TODOs

- More thorough unit testing
- Cute moogle logo.
- IntoIterator for &T for all builtin `.iter()`-providing Ts.
- Add `arr[x]` syntax for all builtin junctions.
- Support miri
- Allow multiple junctions to share a database and allow Prolog-like querying across that database.
- Add ToVecs which behave like ToSets except:
  - tracks and evicts based on insertion order
  - duplicate items are allowed
  - can be capped
  - Consider: ToVec-based junctions are no longer unique in what pairs they contain?
  - Consider: ToOrderedSet instead, or ToIxd? (indexed, but keeping the pairwise uniqueness guarantee)
- Use `BTreeSet<(X, Y)>` instead of `BTreeMap<X, BTreeSet<Y>>`