use super::SetToSet;

use crate::id::IdLike;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::raw_junctions::set_to_set::RawSetToSet;

use std::collections::BTreeSet;

use crate::moogcell::InteriorVSet;
use crate::iterators::{KeysIterator, SetIterator};

use crate::structures::VSet;

// == type ==
pub struct Bwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a SetToSet<A, B> }
pub struct BwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_junctions) parent: &'a SetToSet<A, B>, 
    cache: InteriorVSet<'a, RawSetToSet<A, B>, B, A>,
    pub(in crate::shared_junctions) key: B 
}

// == caching ==
impl <'a, A: IdLike, B: IdLike> BwdSet<'a, A, B> {
    pub(in crate::shared_junctions) fn fetch(&self) -> VSet<'a, B, A> {
        return self.cache.get_or_compute_arg(|o| o.bwd().get_short(self.key).0)
    }
}

// == main impl ==
impl <'a, A: IdLike, B: IdLike> SharedAnyToSet<'a, B, A> for Bwd<'a, A, B> {
    type Multi = BwdSet<'a, A, B>;
    type Expunge = BTreeSet<A>;  

    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(B, Self::Multi)>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> Self::Multi { BwdSet { 
        parent: self.me, 
        cache: self.me.raw.create_interior_vset::<B, A>(), 
        key: b 
    } }
    fn contains_key(&self, b: B) -> bool { self.me.raw.borrow().bwd().contains_key(b) }

    fn len(&self) -> usize { self.me.raw.borrow().bwd().len() }  
    fn keys_len(&self) -> usize { self.me.raw.borrow().bwd().keys_len() }

    fn contains(&'a self, b: B, a: A) -> bool { self.me.raw.borrow().bwd().get(b).contains(a) }

    fn iter(&self) -> Self::Iter {
        let me = self.me;
        self.keys().flat_map(move |k| me.bwd().get(k).iter().map(move |v| (k, v)))
    }
    fn keys(&self) -> Self::Keys {
        BwdKeysIterator::<'a, A, B> { 
            iter: KeysIterator::new(self.me.raw.create_interior_btreeset_range())
        }
    }
    fn sets(&self) -> Self::Sets { 
        let me = self.me;
        self.keys().map(move |k| (k, me.bwd().get(k))) 
    }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, b: B, a: A) -> Option<A> { self.me.raw.borrow_mut().mut_bwd().insert(b, a) }
    fn expunge(&self, b: B) -> Self::Expunge { self.me.raw.borrow_mut().mut_bwd().expunge(b) }
}

// == Forward (sets) ==
impl <'a, A: IdLike, B: IdLike> SharedSet<'a, A> for BwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.fetch().contains(a) }
    fn len(&self) -> usize { self.fetch().len() }

    fn iter(&self) -> Self::Iter {
        BwdSetIterator {
            iter: SetIterator::new(
                self.parent.raw.create_interior_vset(),
                self.parent.raw.create_interior_btreeset_range(),
                self.key,
            )
        }
    }

    fn insert(&self, a: A) -> Option<A> { self.parent.raw.borrow_mut().mut_bwd().get_mut(self.key).insert(a) }
    fn remove(&self, a: A) -> Option<A> { self.parent.raw.borrow_mut().mut_bwd().get_mut(self.key).remove(a) }
}

// == iterators ==
struct BwdKeysIterator<'a, A: IdLike, B: IdLike> {
    iter: KeysIterator<'a, RawSetToSet<A, B>, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for BwdKeysIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next(|p| &p.bwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for BwdKeysIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.bwd)
    }
}

struct BwdSetIterator<'a, A: IdLike, B: IdLike> {
    iter: SetIterator<'a, RawSetToSet<A, B>, B, A>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for BwdSetIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p, k| p.bwd().get_short(k).0)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for BwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p, k| p.bwd().get_short(k).0)
    }
}