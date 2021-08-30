// NOTE: These test modules are written in a way that's intended to make use of rustc
// Specifically, if you see "unused import", you can probably bet that the property listed here
// is not provided (or at least not tested for)
// This lets you spot-check what each bimap has
mod fixture;
use crate::RawPom as T;

use std::collections::VecDeque;

use fixture::*;

impl crate::RawPom<String> {
    fn prepare(fun: &Routine) -> Self {
        let mut pom = Self::new();
        let mut ids = VecDeque::new();
        
        for op in fun.0.iter() {
            match op {
                Op::Insert(s) => { ids.push_back(pom.insert(s.clone())); }
                Op::Remove => {
                    let id = ids.pop_front();
                    match id {
                        None => {}
                        Some(x) => { pom.remove(x); }
                    }
                }
                Op::Append(c) => {
                    let id = ids.pop_front();
                    match id {
                        None => {}
                        Some(x) => { 
                            (*pom.get_mut(x).unwrap()).push_str(&c);
                            ids.push_back(x)
                        }
                    }
                }
                Op::Shuffle(n) => {
                    if !ids.is_empty() {
                        for _ in 0..*n {
                            let x = ids.pop_front().unwrap();
                            ids.push_back(x)
                        }
                    }
                }
            }
        }

        pom
    }
}

#[quickcheck]
fn test_dont_panic(f: Routine) -> bool {
    let _= T::prepare(&f);
    true
}