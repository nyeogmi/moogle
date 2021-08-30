use crate::id::IdLike;

use crate::raw_structures::RawToSet;
use crate::moogcell::MoogCell;

use crate::methods::{SharedAnyToSet, SharedAnySet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, AnySet};

use std::collections::BTreeSet;

use crate::iterators::{ToSetKeysIterator, ToSetKeyValueIterator};

// == Data structure ==
pub struct ToSet<A: IdLike, B: IdLike> {
    pub(in crate::shared_structures) raw: MoogCell<RawToSet<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> ToSet<A, B> {
    pub fn new() -> ToSet<A, B> {
        ToSet { raw: MoogCell::new(RawToSet::new()) }
    }

    pub fn raw(&mut self) -> &mut RawToSet<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
}


// == type ==
pub struct Fwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_structures) me: &'a ToSet<A, B> }
pub struct FwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_structures) parent: &'a ToSet<A, B>, 
    pub(in crate::shared_structures) key: A 
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
                self.me.raw.create_interior_set_range(),
                None, None
            )
        }
    }
    fn keys(&self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            iter: ToSetKeysIterator::new(self.me.raw.create_interior_set_range())
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
impl <'a, A: IdLike, B: IdLike> SharedAnySet<'a, B> for FwdSet<'a, A, B> {
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
                self.parent.raw.create_interior_set_range(),
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
    iter: ToSetKeyValueIterator<'a, RawToSet<A, B>, A, B>,
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
    iter: ToSetKeysIterator<'a, RawToSet<A, B>, A>,
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
    iter: ToSetKeyValueIterator<'a, RawToSet<A, B>, A, B>,
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