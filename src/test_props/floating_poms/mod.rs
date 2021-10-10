// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
mod fixture;
mod properties;
use crate::FloatingPom as T;
use crate::Id;

use fixture::*;

use std::collections::VecDeque;

impl crate::FloatingPom<String> {
    fn prepare(fun: &Routine) -> Self {
        let mut pom = Self::new();
        let mut ids = VecDeque::new();
        for phase in &fun.phases {
            pom.prepare_phase(phase, &mut ids);
        }
        pom
    }

    fn prepare_phase(&mut self, phase: &Phase, ids: &mut VecDeque<Id<String>>) {
        self.prepare_unshared(&phase.0, ids);
        self.prepare_shared(&phase.1, ids);
    }

    fn prepare_unshared(&mut self, ops: &[UnsharedOp], ids: &mut VecDeque<Id<String>>) {
        for op in ops {
            match op {
                UnsharedOp::Insert(s) => { ids.push_back(self.insert(s.clone())); }
                UnsharedOp::Remove => {
                    let id = ids.pop_front();
                    match id {
                        None => {}
                        Some(x) => { self.remove(x); }
                    }
                }
                UnsharedOp::Append(c) => {
                    let id = ids.pop_front();
                    match id {
                        None => {}
                        Some(x) => { 
                            (*self.get(x).unwrap().borrow_mut()).push_str(&c);
                            ids.push_back(x)
                        }
                    }
                }
                UnsharedOp::Shuffle(n) => {
                    if !ids.is_empty() {
                        for _ in 0..*n {
                            let x = ids.pop_front().unwrap();
                            ids.push_back(x)
                        }
                    }
                }
            }
        }
    }

    fn prepare_shared(&mut self, ops: &[SharedOp], ids: &mut VecDeque<Id<String>>) {
        let mut iterator = self.keys();

        // TODO: Crash if index's ids != ids
        for op in ops {
            match op {
                SharedOp::Append(c) => {
                    let id = ids.pop_front();
                    match id {
                        None => {}
                        Some(x) => {
                            (*self.get(x).unwrap().borrow_mut()).push_str(&c);
                            ids.push_back(x);
                        }
                    }
                }
                SharedOp::Shuffle(n) => {
                    if !ids.is_empty() {
                        for _ in 0..*n {
                            let x = ids.pop_front().unwrap();
                            ids.push_back(x)
                        }
                    }
                }
                SharedOp::IterateKeys => {
                    let mut sorted_known_ids: Vec<Id<String>> = ids.iter().cloned().collect();
                    sorted_known_ids.sort();
                    // assert index agrees on ids we have
                    let index_ids: Vec<Id<String>> = iterator.collect();
                    assert_eq!(index_ids, sorted_known_ids);

                    let elements_ids: Vec<Id<String>> = self.keys().collect();
                    assert_eq!(index_ids, elements_ids);
                    
                    // build a new iterator
                    iterator = self.keys();
                }
            }
        }
    }
}

#[quickcheck]
fn test_dont_panic(f: Routine) -> bool {
    let _= T::prepare(&f);
    true
}