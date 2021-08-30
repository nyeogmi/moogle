use crate::test_props::setty_fixture::*;
use crate::methods::*;
use crate::test_props::setty_properties::{unique, correct_len};
use crate::RawSet as T; 

impl crate::RawSet<u16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{opts} => {
                    for t in items_only(opts) { set.mut_fwd().insert(t); }
                },
                Phase::Remove{opts} => {
                    for t in items_only(opts) { set.mut_fwd().remove(t); }
                },
            }
        }
        set
    }
}

#[quickcheck]
fn test_unique(f: Routine) -> bool {
    let xs = T::prepare(&f);
    unique(xs.fwd().iter().collect())
}

#[quickcheck]
fn test_correct_len(f: Routine) -> bool {
    let xs = T::prepare(&f);
    correct_len(xs.fwd().iter().collect(), xs.fwd().len())
}