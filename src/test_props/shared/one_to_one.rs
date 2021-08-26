// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
use crate::test_props::fixture::*;
use crate::methods::*;
use crate::test_props::properties::{symmetrical, fwd_correct_len, bwd_correct_len};
use crate::test_props::properties::{pair_unique, fwd_unique, bwd_unique}; 
use crate::OneToOne as T;

use crate::test_props::iterbank::{IterBank, wrap};

impl crate::OneToOne<u16, i16> {
    fn iterator_maker<'a>(&'a self) -> Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>> {
        vec![
            wrap(self.fwd().keys()), wrap(self.bwd().keys()),
            wrap(self.fwd().iter()), wrap(self.bwd().iter()),
            wrap(self.fwd().values()), wrap(self.bwd().values()),
        ]
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
                Phase::Insert{fwd, bwd} => {
                    items_or_work(fwd, |(a, b)| { set.fwd().insert(a, b); }, |w| iter_bank.do_work(w));
                    items_or_work(bwd, |(b, a)| { set.bwd().insert(b, a); }, |w| iter_bank.do_work(w));
                },
                Phase::Remove{fwd, bwd} => {
                    items_or_work(fwd, |(a, b)| { set.fwd().remove(a, b); }, |w| iter_bank.do_work(w));
                    items_or_work(bwd, |(b, a)| { set.bwd().remove(b, a); }, |w| iter_bank.do_work(w));
                },
                Phase::Expunge{fwd, bwd} => {
                    items_or_work(fwd, |a| { set.fwd().expunge(a); }, |w| iter_bank.do_work(w));
                    items_or_work(bwd, |b| { set.bwd().expunge(b); }, |w| iter_bank.do_work(w));
                }
            }
        }

        std::mem::drop(iter_bank);
        set
    }
}

#[quickcheck]
fn test_fwd_correct_len(f: Routine) -> bool {
    let xs = T::prepare(&f);
    fwd_correct_len(xs.fwd().iter().collect(), xs.fwd().len())
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

#[quickcheck]
fn test_fwd_unique(f: Routine) -> bool {
    let xs = T::prepare(&f);
    fwd_unique(xs.fwd().iter().collect())
}

#[quickcheck]
fn test_bwd_unique(f: Routine) -> bool {
    let xs = T::prepare(&f);
    bwd_unique(xs.bwd().iter().collect())
}