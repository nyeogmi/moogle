use std::cell::RefCell;

use crate::keybound::Id;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set, EvictSet};

use crate::junctions::set_to_set::{SetToSet, VFwdSet, VBwdSet};

use std::collections::BTreeSet;

use super::moogcell::{MoogCell, InteriorSetRange, InteriorTreeRange, InteriorVSet};

use crate::structures::VSet;

// == Data structure ==
pub struct SharedSetToSet<A: Id, B: Id> {
    raw: MoogCell<SetToSet<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedSetToSet<A, B> {
    pub fn new() -> SharedSetToSet<A, B> {
        SharedSetToSet { raw: MoogCell::new(SetToSet::new()) }
    }
}

// == More structs ==
pub struct Fwd<'a, A: Id, B: Id> { me: &'a SharedSetToSet<A, B> }
pub struct FwdSet<'a, A: Id, B: Id> { 
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, A, B>,
    key: A 
}
pub struct Bwd<'a, A: Id, B: Id> { me: &'a SharedSetToSet<A, B> }
pub struct BwdSet<'a, A: Id, B: Id> { 
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, B, A>,
    key: B 
}

// == Caching ==
impl <'a, A: Id, B: Id> FwdSet<'a, A, B> {
    fn fetch(&self) -> VSet<'a, A, B> {
        return self.cache.get_or_compute(
            &self.parent.raw, 
            |o| o.fwd().get_short(self.key).0,
        )
    }
}

// == Accessors ==
impl <A: Id, B: Id> SharedSetToSet<A, B> {
    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}

// == Forward ==
impl <'a, A: Id, B: Id> SharedAnyToSet<'a, A, B> for Fwd<'a, A, B> {
    type Multi = FwdSet<'a, A, B>;
    type Expunge = BTreeSet<B>;  

    type Iter = impl 'a+Iterator<Item=(A, B)>;
    type Keys = impl 'a+Iterator<Item=A>;
    type Sets = impl 'a+Iterator<Item=(A, Self::Multi)>;
    type Values = impl 'a+Iterator<Item=B>;

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
        // TODO: This is probably not a very fast implementation
        self.keys().flat_map(move |k| self.get(k).iter().map(move |v| (k, v)))
    }
    fn keys(&'a self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            me: self.me,
            iterator: self.me.raw.create_interior_tree_range(),
            last_key: None,
        }
    }
    fn sets(&'a self) -> Self::Sets { self.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Self::Expunge { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == Forward (sets) ==
impl <'a, A: Id, B: Id> SharedSet<'a, B> for FwdSet<'a, A, B> {
    type Iter = impl 'a+Iterator<Item=B>;

    fn contains(&self, b: B) -> bool { self.fetch().contains(b) }
    fn len(&self) -> usize { self.fetch().len() }

    fn iter(&self) -> Self::Iter {
        FwdSetIterator::<'a, A, B> { 
            parent: self.parent, 
            cache: self.cache.clone(), 
            iterator: self.parent.raw.create_interior_set_range(),

            key: self.key,
            last_value: None,
        }
    }

    fn insert(&self, b: B) -> Option<B>  { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).insert(b) }
    fn remove(&self, b: B) -> Option<B> { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).remove(b) }
}

// == Forward (iterators) ==
struct FwdKeysIterator<'a, A: Id, B: Id> {
    me: &'a SharedSetToSet<A, B>, 
    iterator: InteriorTreeRange<SetToSet<A, B>, A, BTreeSet<B>>,

    last_key: Option<A>,
}

impl<'a, A: Id, B: Id> Iterator for FwdKeysIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        let lk = self.last_key;
        let iter = self.iterator.get_or_compute(&self.me.raw, |xs| {
            match lk {
                Some(x) => {let mut l = xs.fwd.range(x..); l.next(); l}
                None => xs.fwd.range(..),
            }
        });
        let k = iter.next().map(|(k, _)| *k); self.last_key = k; k
    }
}

struct FwdSetIterator<'a, A: Id, B: Id> {
    parent: &'a SharedSetToSet<A, B>, 
    cache: InteriorVSet<SetToSet<A, B>, A, B>,
    iterator: InteriorSetRange<SetToSet<A, B>, B>,

    key: A,
    last_value: Option<B>,
}

impl<'a, A: Id, B: Id> Iterator for FwdSetIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        let lv = self.last_value;
        let key = self.key;

        let set = self.cache.get_or_compute(&self.parent.raw, |xs| {
            xs.fwd().get_short(key).0
        });
        let bt = match set.0 {
            None => return None,
            Some(b) => b,
        };
        let iterator = self.iterator.get_or_compute(&self.parent.raw, || {
            match lv {
                Some(x) => {let mut l = bt.range(x..); l.next(); l}
                None => bt.range(..)
            }
        });
        let v = iterator.next().map(|v| *v); self.last_value = v; v
    }
}