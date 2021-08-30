use crate::id::IdLike;

use crate::raw_structures::RawSet;
use crate::moogcell::MoogCell;

use crate::methods::{SharedAnySet};
use crate::methods::{ViewSet, AnySet};

use crate::iterators::{BTreeMapIterator};

// == Data structure ==
pub struct Set<A: IdLike> {
    pub(in crate::shared_structures) raw: MoogCell<RawSet<A>>
}

// == Constructor et al ==
impl<A: IdLike> Set<A> {
    pub fn new() -> Set<A> {
        Set { raw: MoogCell::new(RawSet::new()) }
    }

    pub fn raw(&mut self) -> &mut RawSet<A> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A> { Fwd { me: self } }
}

// == type ==
pub struct Fwd<'a, A: IdLike> { pub(in crate::shared_structures) me: &'a Set<A> }

// == main impl ==
impl <'a, A: IdLike> SharedAnySet<'a, A> for Fwd<'a, A> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains(a) }
    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }

    fn iter(&self) -> Self::Iter {
        FwdFlatIterator::<'a, A> {
            iter: BTreeMapIterator::new(self.me.raw.create_interior_map_range()),
        }
    }

    fn insert(&self, a: A) -> Option<A> { self.me.raw.borrow_mut().mut_fwd().insert(a) }
    fn remove(&self, a: A) -> Option<A> { self.me.raw.borrow_mut().mut_fwd().remove(a) }

}

// == iterators ==
struct FwdFlatIterator<'a, A: IdLike> {
    iter: BTreeMapIterator<'a, RawSet<A>, A, ()>,
}

impl<'a, A: IdLike> Iterator for FwdFlatIterator<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p| &p.underlying.fwd.0).map(|(k, _)| k)
    }
}

impl <'a, A: IdLike> DoubleEndedIterator for FwdFlatIterator<'a, A> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.underlying.fwd.0).map(|(k, _)| k)
    }
}