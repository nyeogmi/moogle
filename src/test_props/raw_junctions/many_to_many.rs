// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
use crate::test_props::mappy_fixture::*;
use crate::methods::*;
use crate::test_props::mappy_properties::{symmetrical, fwd_equal, fwd_correct_len, bwd_equal, bwd_correct_len};
use crate::test_props::mappy_properties::pair_unique; 
use crate::RawManyToMany as T;

impl crate::RawManyToMany<u16, i16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{fwd, bwd} => {
                    for (a, b) in items_only(fwd) { set.mut_fwd().insert(a, b); }
                    for (b, a) in items_only(bwd) { set.mut_bwd().insert(b, a); }
                },
                Phase::Remove{fwd, bwd} => {
                    for (a, b) in items_only(fwd) { set.mut_fwd().remove(a, b); }
                    for (b, a) in items_only(bwd) { set.mut_bwd().remove(b, a); }
                },
                Phase::Expunge{fwd, bwd} => {
                    for a in items_only(fwd) { set.mut_fwd().expunge(a); }
                    for b in items_only(bwd) { set.mut_bwd().expunge(b); }
                }
            }
        }
        set
    }
}

#[quickcheck]
fn test_fwd_equal(f: Routine) -> bool {
    let mut xs = T::prepare(&f);
    fwd_equal(xs.fwd().iter().collect(), xs.mut_fwd().iter().collect())
}

#[quickcheck]
fn test_fwd_correct_len(f: Routine) -> bool {
    let xs = T::prepare(&f);
    fwd_correct_len(xs.fwd().iter().collect(), xs.fwd().len())
}

#[quickcheck]
fn test_bwd_equal(f: Routine) -> bool {
    let mut xs = T::prepare(&f);
    bwd_equal(xs.bwd().iter().collect(), xs.mut_bwd().iter().collect())
}

#[quickcheck]
fn test_bwd_correct_len(f: Routine) -> bool {
    let xs = T::prepare(&f);
    bwd_correct_len(xs.bwd().iter().collect(), xs.bwd().len())
}

#[quickcheck]
fn test_symmetrical(f: Routine) -> bool {
    let xs = T::prepare(&f);
    symmetrical(xs.fwd().iter().collect(), xs.bwd().iter().collect())
}

#[quickcheck]
fn test_pair_unique(f: Routine) -> bool {
    let xs = T::prepare(&f);
    pair_unique(xs.fwd().iter().collect())
}