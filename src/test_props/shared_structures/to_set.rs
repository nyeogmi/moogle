// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
use crate::test_props::fixture::*;
use crate::methods::*;
use crate::test_props::properties::fwd_correct_len;
use crate::test_props::properties::pair_unique; 
use crate::ToSet as T;

use crate::test_props::iterbank::{IterBank, wrap, DESIRED_N_ITERATORS};

impl crate::ToSet<u16, i16> {
    fn iterator_maker<'a>(&'a self) -> Vec<Box<dyn 'a+DoubleEndedIterator<Item=()>>> {
        vec![
            wrap(self.fwd().keys()), 
            wrap(self.fwd().iter()), 
            wrap(self.fwd().sets()), 
            wrap(self.fwd().values()), 
        ]
    }
    
    fn prime<'a>(&'a self, phase: &Phase, iter_bank: &mut IterBank<'a>) {
        let (items1, items2): (Vec<u16>, Vec<u16>) = match phase {
            Phase::Insert{fwd, bwd} => (
                items_only(&fwd).iter().take(DESIRED_N_ITERATORS / 2).map(|(a, _)| *a).collect(),
                items_only(&bwd).iter().take(DESIRED_N_ITERATORS / 2).map(|(_, a)| *a).collect(),
            ),
            Phase::Remove{fwd, bwd} => (
                items_only(&fwd).iter().take(DESIRED_N_ITERATORS / 2).map(|(a, _)| *a).collect(),
                items_only(&bwd).iter().take(DESIRED_N_ITERATORS / 2).map(|(_, a)| *a).collect(),
            ),
            Phase::Expunge{fwd, ..} => (
                items_only(&fwd).into_iter().take(DESIRED_N_ITERATORS).collect(),
                vec![],
            )
        };
        for i in items1 { iter_bank.add_iterator(self.fwd().get(i).iter()) }
        for i in items2 { iter_bank.add_iterator(self.fwd().get(i).iter()) }
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
                    items_or_work(bwd, |(b, a)| { set.fwd().insert(a, b); }, |w| iter_bank.do_work(w));
                },
                Phase::Remove{fwd, bwd} => {
                    items_or_work(fwd, |(a, b)| { set.fwd().remove(a, b); }, |w| iter_bank.do_work(w));
                    items_or_work(bwd, |(b, a)| { set.fwd().remove(a, b); }, |w| iter_bank.do_work(w));
                },
                Phase::Expunge{fwd, ..} => {
                    items_or_work(fwd, |a| { set.fwd().expunge(a); }, |w| iter_bank.do_work(w));
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
fn test_pair_unique(f: Routine) -> bool {
    let xs = T::prepare(&f);
    pair_unique(xs.fwd().iter().collect())
}