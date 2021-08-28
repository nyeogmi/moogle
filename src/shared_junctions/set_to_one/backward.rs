use super::SetToOne;

use crate::id::IdLike;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::raw_junctions::set_to_one::RawSetToOne;

use std::collections::BTreeSet;

use crate::moogcell::InteriorVSet;
use crate::iterators::{ToSetKeysIterator, ToSetKeyValueIterator, VSetIterator};

use crate::structures::VSet;

// == type ==
pub struct Bwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a SetToOne<A, B> }
pub struct BwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_junctions) parent: &'a SetToOne<A, B>, 
    cache: InteriorVSet<'a, RawSetToOne<A, B>, B, A>,
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

    fn iter(&self) -> Self::Iter {
        BwdIterator::<'a, A, B> {
            iter: ToSetKeyValueIterator::new(self.me.raw.create_interior_btreeset_range())
        }
    }
    fn keys(&self) -> Self::Keys {
        BwdKeysIterator::<'a, A, B> { 
            iter: ToSetKeysIterator::new(self.me.raw.create_interior_btreeset_range())
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
            iter: VSetIterator::new(
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
struct BwdIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeyValueIterator<'a, RawSetToOne<A, B>, B, A>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for BwdIterator<'a, A, B> {
    type Item = (B, A);

    fn next(&mut self) -> Option<(B, A)> {
        self.iter.next(|p| &p.bwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for BwdIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.bwd)
    }
}
struct BwdKeysIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeysIterator<'a, RawSetToOne<A, B>, B>,
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
    iter: VSetIterator<'a, RawSetToOne<A, B>, B, A>,
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