
## TODOs

- Cute moogle logo.
- IntoIterator for &T for all builtin `.iter()`-providing Ts.
- Use the length as in "number of pairs" for len() in ToSets. (rather than "number of keys," which is used now) To do this, I'll need to calculate total length.
- Allow an iterator to stay alive during changes to the underlying object (by tracking the last key hit and using BTree operations)
- Allow multiple junctions to share a database and allow Prolog-like querying across that database.
- Remove dependencies on macro code (by writing iterators manually)