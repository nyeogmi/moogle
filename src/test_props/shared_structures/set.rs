// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
use crate::test_props::setty_fixture::*;
use crate::methods::*;
use crate::test_props::setty_properties::{unique, correct_len};
use crate::Set as T;

use crate::test_props::iterbank::{IterBank, wrap};

impl crate::Set<u16> {
    fn iterator_maker<'a>(&'a self) -> Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>> {
        vec![ wrap(self.fwd().iter()), ]
    }
    
    fn prime<'a>(&'a self, _phase: &Phase, _iter_bank: &mut IterBank<'a>) {
        // do nothing: no internal iterators
    }

    fn prepare(fun: &Routine) -> Self {
        let set = Self::new();
        let mut iter_bank = IterBank::new(|| set.iterator_maker());
        
        for (i, phase) in fun.0.iter().enumerate() {
            // prime every _other_ phase
            if i % 2 == 1 { set.prime(phase, &mut iter_bank) }

            match phase {
                Phase::Insert{opts} => {
                    items_or_work(opts, |a| { set.fwd().insert(a); }, |w| iter_bank.do_work(w));
                },
                Phase::Remove{opts} => {
                    items_or_work(opts, |a| { set.fwd().remove(a); }, |w| iter_bank.do_work(w));
                },
            }
        }

        std::mem::drop(iter_bank);
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