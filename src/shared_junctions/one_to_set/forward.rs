use super::OneToSet;

use crate::id::IdLike;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::raw_junctions::one_to_set::RawOneToSet;

use std::collections::BTreeSet;

use crate::iterators::{ToSetKeysIterator, ToSetKeyValueIterator};

// == type ==
pub struct Fwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a OneToSet<A, B> }
pub struct FwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_junctions) parent: &'a OneToSet<A, B>, 
    pub(in crate::shared_junctions) key: A 
}

// == main impl ==
impl <'a, A: IdLike, B: IdLike> SharedAnyToSet<'a, A, B> for Fwd<'a, A, B> {
    type Multi = FwdSet<'a, A, B>;
    type Expunge = BTreeSet<B>;  

    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(A, Self::Multi)>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Self::Multi { FwdSet { 
        parent: self.me, 
        key: a 
    } }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }

    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }  
    fn keys_len(&self) -> usize { self.me.raw.borrow().fwd().keys_len() }

    fn iter(&self) -> Self::Iter {
        FwdIterator::<'a, A, B> {
            iter: ToSetKeyValueIterator::new(
                self.me.raw.create_interior_btreeset_range(),
                None, None
            )
        }
    }
    fn keys(&self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            iter: ToSetKeysIterator::new(self.me.raw.create_interior_btreeset_range())
        }
    }
    fn sets(&self) -> Self::Sets { 
        let me = self.me;
        self.keys().map(move |k| (k, me.fwd().get(k))) 
    }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Self::Expunge { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == Forward (sets) ==
impl <'a, A: IdLike, B: IdLike> SharedSet<'a, B> for FwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=B>;

    fn contains(&self, b: B) -> bool { 
        let parent = self.parent.raw.borrow();
        let fetch = parent.fwd.get(self.key);
        fetch.contains(b) 
    }
    fn len(&self) -> usize { 
        let parent = self.parent.raw.borrow();
        let fetch = parent.fwd.get(self.key);
        fetch.len() 
    }

    fn iter(&self) -> Self::Iter {
        FwdSetIterator {
            iter: ToSetKeyValueIterator::new(
                self.parent.raw.create_interior_btreeset_range(),
                Some((self.key, B::id_min_value())), 
                Some((self.key, B::id_max_value())),
            )
        }
    }

    fn insert(&self, b: B) -> Option<B>  { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).insert(b) }
    fn remove(&self, b: B) -> Option<B> { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).remove(b) }
}

// == iterators ==
struct FwdIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeyValueIterator<'a, RawOneToSet<A, B>, A, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdIterator<'a, A, B> {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}

struct FwdKeysIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeysIterator<'a, RawOneToSet<A, B>, A>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdKeysIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdKeysIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}

struct FwdSetIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeyValueIterator<'a, RawOneToSet<A, B>, A, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdSetIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next(|p| &p.fwd).map(|(_, v)| v)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd).map(|(_, v)| v)
    }
}