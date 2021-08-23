fn symmetrical(
    fwd: Vec<(u16, i16)>, 
    bwd: Vec<(i16, u16)>
) -> bool {
    // forward = {(a, b) for (b, a) in backward}
    let mut bwd_sort: Vec<(u16, i16)> = bwd.iter().map(|(x, y)| (*y, *x)).collect();
    bwd_sort.sort();
    fwd == bwd_sort
}

fn pair_unique(
    fwd: Vec<(u16, i16)>
) -> bool {
    use std::iter::FromIterator;

    // no duplicate pairs
    std::collections::BTreeSet::from_iter(fwd.iter()).len() == fwd.len() 
}
fn fwd_unique(
    fwd: Vec<(u16, i16)>, 
) -> bool {
    use std::iter::FromIterator;

    // no duplicates period
    // no duplicate u16s
    std::collections::BTreeSet::from_iter(fwd.iter().map(|(x, _)| x)).len() == fwd.len()
}

fn bwd_unique(
    bwd: Vec<(i16, u16)>, 
) -> bool {
    use std::iter::FromIterator;
    // no duplicate i16s
    std::collections::BTreeSet::from_iter(bwd.iter().map(|(x, _)| x)).len() == bwd.len()
}

fn fwd_equal(fwd: Vec<(u16, i16)>, mut_fwd: Vec<(u16, i16)>) -> bool {
    fwd == mut_fwd
}

fn bwd_equal(bwd: Vec<(i16, u16)>, mut_bwd: Vec<(i16, u16)>) -> bool {
    bwd == mut_bwd
}

#[derive(Clone, Debug)]
enum Phase {
    Insert { fwd: Vec<(u16, i16)>, bwd: Vec<(i16, u16)> },
    Expunge { fwd: Vec<u16>, bwd: Vec<i16> },
    Remove { fwd: Vec<(u16, i16)>, bwd: Vec<(i16, u16)> },
}

#[derive(Clone, Debug)]
struct Routine(Vec<Phase>);

// TODO: Test removal and expunging too
impl quickcheck::Arbitrary for Routine {
    fn arbitrary(g: &mut quickcheck::Gen) -> Routine { 
        let length: i16 = *g.choose(&[0, 1, 1, 2, 2, 2, 3, 3, 4, 5]).unwrap();
        let mut xs = vec![];
        for _ in 0..length {
            xs.push(Phase::arbitrary(g));
        }
        Routine(xs)
    }
}

impl quickcheck::Arbitrary for Phase {
    fn arbitrary(g: &mut quickcheck::Gen) -> Phase { 
        let options: [fn(&mut quickcheck::Gen) -> Phase; 3] = [
            |g_| Phase::Insert { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
            |g_| Phase::Expunge { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
            |g_| Phase::Remove { fwd: Vec::arbitrary(g_), bwd: Vec::arbitrary(g_) },
        ];
        g.choose(&options).unwrap()(g)
    }
}


use crate::methods::*;

impl crate::OneToOne<u16, i16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().insert(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().insert(*b, *a); }
                },
                Phase::Remove{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().remove(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().remove(*b, *a); }
                },
                Phase::Expunge{fwd, bwd} => {
                    for a in fwd { set.mut_fwd().expunge(*a); }
                    for b in bwd { set.mut_bwd().expunge(*b); }
                }
            }
        }
        set
    }
}

impl crate::OneToSet<u16, i16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().insert(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().insert(*b, *a); }
                },
                Phase::Remove{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().remove(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().remove(*b, *a); }
                },
                Phase::Expunge{fwd, bwd} => {
                    for a in fwd { set.mut_fwd().expunge(*a); }
                    for b in bwd { set.mut_bwd().expunge(*b); }
                }
            }
        }
        set
    }
}

impl crate::SetToOne<u16, i16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().insert(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().insert(*b, *a); }
                },
                Phase::Remove{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().remove(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().remove(*b, *a); }
                },
                Phase::Expunge{fwd, bwd} => {
                    for a in fwd { set.mut_fwd().expunge(*a); }
                    for b in bwd { set.mut_bwd().expunge(*b); }
                }
            }
        }
        set
    }
}

impl crate::SetToSet<u16, i16> {
    fn prepare(fun: &Routine) -> Self {
        let mut set = Self::new();
        for phase in &fun.0 {
            match phase {
                Phase::Insert{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().insert(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().insert(*b, *a); }
                },
                Phase::Remove{fwd, bwd} => {
                    for (a, b) in fwd { set.mut_fwd().remove(*a, *b); }
                    for (b, a) in bwd { set.mut_bwd().remove(*b, *a); }
                },
                Phase::Expunge{fwd, bwd} => {
                    for a in fwd { set.mut_fwd().expunge(*a); }
                    for b in bwd { set.mut_bwd().expunge(*b); }
                }
            }
        }
        set
    }
}

mod one_to_one {
    // NOTE: These test modules are written in a way that's intended to make use of rustc
    // Specifically, if you see "unused import", you can probably bet that the property listed here
    // is not provided (or at least not tested for)
    // This lets you spot-check what each bimap has
    use crate::methods::*;
    use super::{Routine, symmetrical, fwd_equal, bwd_equal};
    use super::{pair_unique, fwd_unique, bwd_unique}; 
    use crate::OneToOne as T;
    
    #[quickcheck]
    fn test_fwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        fwd_equal(xs.fwd().iter().collect(), xs.mut_fwd().iter().collect())
    }

    #[quickcheck]
    fn test_bwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        bwd_equal(xs.bwd().iter().collect(), xs.mut_bwd().iter().collect())
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
}

mod one_to_set {
    // NOTE: These test modules are written in a way that's intended to make use of rustc
    // Specifically, if you see "unused import", you can probably bet that the property listed here
    // is not provided (or at least not tested for)
    // This lets you spot-check what each bimap has
    use crate::methods::*;
    use super::{Routine, symmetrical, fwd_equal, bwd_equal};
    use super::{pair_unique, bwd_unique}; 
    use crate::OneToSet as T;
    
    #[quickcheck]
    fn test_fwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        fwd_equal(xs.fwd().iter().collect(), xs.mut_fwd().iter().collect())
    }

    #[quickcheck]
    fn test_bwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        bwd_equal(xs.bwd().iter().collect(), xs.mut_bwd().iter().collect())
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
    fn test_bwd_unique(f: Routine) -> bool {
        let xs = T::prepare(&f);
        bwd_unique(xs.bwd().iter().collect())
    }
}

mod set_to_one {
    // NOTE: These test modules are written in a way that's intended to make use of rustc
    // Specifically, if you see "unused import", you can probably bet that the property listed here
    // is not provided (or at least not tested for)
    // This lets you spot-check what each bimap has
    use crate::methods::*;
    use super::{Routine, symmetrical, fwd_equal, bwd_equal};
    use super::{pair_unique, fwd_unique}; 
    use crate::SetToOne as T;
    
    #[quickcheck]
    fn test_fwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        fwd_equal(xs.fwd().iter().collect(), xs.mut_fwd().iter().collect())
    }

    #[quickcheck]
    fn test_bwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        bwd_equal(xs.bwd().iter().collect(), xs.mut_bwd().iter().collect())
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
}

mod set_to_set {
    // NOTE: These test modules are written in a way that's intended to make use of rustc
    // Specifically, if you see "unused import", you can probably bet that the property listed here
    // is not provided (or at least not tested for)
    // This lets you spot-check what each bimap has
    use crate::methods::*;
    use super::{Routine, symmetrical, fwd_equal, bwd_equal};
    use super::{pair_unique}; 
    use crate::SetToSet as T;
    
    #[quickcheck]
    fn test_fwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        fwd_equal(xs.fwd().iter().collect(), xs.mut_fwd().iter().collect())
    }

    #[quickcheck]
    fn test_bwd_equal(f: Routine) -> bool {
        let mut xs = T::prepare(&f);
        bwd_equal(xs.bwd().iter().collect(), xs.mut_bwd().iter().collect())
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
}