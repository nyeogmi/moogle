use super::SharedSetToSet;

use crate::keybound::Id;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::junctions::set_to_set::SetToSet;

use std::collections::{BTreeSet, btree_set, btree_map};

use super::super::range_utils;
use super::super::moogcell::{InteriorSetRange, InteriorTreeRange, InteriorVSet};

use crate::structures::VSet;

// == type ==
pub struct Fwd<'a, A: Id, B: Id> { pub(super) me: &'a SharedSetToSet<A, B> }
pub struct FwdSet<'a, A: Id, B: Id> { 
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, A, B>,
    key: A 
}

// == caching ==
impl <'a, A: Id, B: Id> FwdSet<'a, A, B> {
    fn fetch(&self) -> VSet<'a, A, B> {
        return self.cache.get_or_compute(
            &self.parent.raw, 
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

    fn contains(&'a self, a: A, b: B) -> bool { self.me.raw.borrow().fwd().get(a).contains(b) }

    fn iter(&'a self) -> Self::Iter {
        self.keys().flat_map(move |k| self.get(k).iter().map(move |v| (k, v)))
    }
    fn keys(&'a self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            me: self.me,
            iterator: self.me.raw.create_interior_tree_range(),
            front_cursor: None,
            back_cursor: None,
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
        FwdSetIterator::<'a, A, B> { 
            parent: self.parent, 
            cache: self.cache.clone(), 
            iterator: self.parent.raw.create_interior_set_range(),

            key: self.key,
            front_cursor: None,
            back_cursor: None,
        }
    }

    fn insert(&self, b: B) -> Option<B>  { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).insert(b) }
    fn remove(&self, b: B) -> Option<B> { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).remove(b) }
}

// == iterators ==
struct FwdKeysIterator<'a, A: Id, B: Id> {
    me: &'a SharedSetToSet<A, B>, 
    iterator: InteriorTreeRange<SetToSet<A, B>, A, BTreeSet<B>>,

    front_cursor: Option<A>,
    back_cursor: Option<A>,
}

impl<'a, A: Id, B: Id> FwdKeysIterator<'a, A, B> {
    fn reconstitute(&mut self) -> &mut btree_map::Range<'_, A, BTreeSet<B>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(&self.me.raw, |xs| {
            range_utils::make_toset_range(&xs.fwd, fc, bc)
        });
        iterator
    }
}

impl<'a, A: Id, B: Id> Iterator for FwdKeysIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        let iter = self.reconstitute();
        let k = iter.next().map(|(k, _)| *k); 
        self.front_cursor = k; 
        k
    }
}

impl <'a, A: Id, B: Id> DoubleEndedIterator for FwdKeysIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        let iter = self.reconstitute();
        let k = iter.next_back().map(|(k, _)| *k); 
        self.back_cursor = k; 
        k
    }
}

struct FwdSetIterator<'a, A: Id, B: Id> {
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, A, B>,
    iterator: InteriorSetRange<SetToSet<A, B>, B>,

    key: A,
    front_cursor: Option<B>,
    back_cursor: Option<B>,
}

impl<'a, A: Id, B: Id> FwdSetIterator<'a, A, B> {
    fn reconstitute(&mut self) -> Option<&mut btree_set::Range<'_, B>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let key = self.key;

        let set = self.cache.get_or_compute(&self.parent.raw, |xs| {
            xs.fwd().get_short(key).0
        });
        let bt = match set.0 {
            None => return None,
            Some(b) => b,
        };
        let iterator = self.iterator.get_or_compute(&self.parent.raw, || {
            range_utils::make_btreeset_range(bt, fc, bc)
        });
        Some(iterator)
    }
}

impl<'a, A: Id, B: Id> Iterator for FwdSetIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        let iterator = self.reconstitute()?;
        let v = iterator.next().map(|v| *v); 
        self.front_cursor = v; 
        v
    }
}

impl <'a, A: Id, B: Id> DoubleEndedIterator for FwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        let iterator = self.reconstitute()?;
        let v = iterator.next_back().map(|v| *v); 
        self.back_cursor = v; 
        v
    }
}