use super::SharedSetToSet;

use crate::keybound::Id;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::raw_junctions::set_to_set::RawSetToSet;

use std::collections::BTreeSet;

use super::super::moogcell::InteriorVSet;
use super::super::iterators::{KeysIterator, SetIterator};

use crate::structures::VSet;

// == type ==
pub struct Fwd<'a, A: Id, B: Id> { pub(super) me: &'a SharedSetToSet<A, B> }
pub struct FwdSet<'a, A: Id, B: Id> { 
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<'a, RawSetToSet<A, B>, A, B>,
    key: A 
}

// == caching ==
impl <'a, A: Id, B: Id> FwdSet<'a, A, B> {
    fn fetch(&self) -> VSet<'a, A, B> {
        return self.cache.get_or_compute(
            |o| o.fwd().get_short(self.key).0,
        )
    }
}

// == main impl ==
impl <'a, A: Id, B: Id> SharedAnyToSet<'a, A, B> for Fwd<'a, A, B> {
    type Multi = FwdSet<'a, A, B>;
    type Expunge = BTreeSet<B>;  

    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(A, Self::Multi)>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Self::Multi { FwdSet { 
        parent: self.me, 
        cache: self.me.raw.create_interior_vset::<A, B>(), 
        key: a 
    } }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }

    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }  
    fn keys_len(&self) -> usize { self.me.raw.borrow().fwd().keys_len() }

    fn iter(&'a self) -> Self::Iter {
        self.keys().flat_map(move |k| self.get(k).iter().map(move |v| (k, v)))
    }
    fn keys(&'a self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            iter: KeysIterator::new(self.me.raw.create_interior_tree_range())
        }
    }
    fn sets(&'a self) -> Self::Sets { self.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Self::Expunge { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == Forward (sets) ==
impl <'a, A: Id, B: Id> SharedSet<'a, B> for FwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=B>;

    fn contains(&self, b: B) -> bool { self.fetch().contains(b) }
    fn len(&self) -> usize { self.fetch().len() }

    fn iter(&self) -> Self::Iter {
        FwdSetIterator {
            iter: SetIterator::new(
                self.parent.raw.create_interior_vset(),
                self.parent.raw.create_interior_set_range(),
                self.key,
            )
        }
    }

    fn insert(&self, b: B) -> Option<B>  { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).insert(b) }
    fn remove(&self, b: B) -> Option<B> { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).remove(b) }
}

// == iterators ==
struct FwdKeysIterator<'a, A: Id, B: Id> {
    iter: KeysIterator<'a, RawSetToSet<A, B>, A, B>,
}

impl<'a, A: Id, B: Id> Iterator for FwdKeysIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: Id, B: Id> DoubleEndedIterator for FwdKeysIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}

struct FwdSetIterator<'a, A: Id, B: Id> {
    iter: SetIterator<'a, RawSetToSet<A, B>, A, B>,
}

impl<'a, A: Id, B: Id> Iterator for FwdSetIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next(|p, k| p.fwd().get_short(k).0)
    }
}

impl <'a, A: Id, B: Id> DoubleEndedIterator for FwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p, k| p.fwd().get_short(k).0)
    }
}