use super::ManyToMany;

use crate::id::IdLike;

use crate::methods::{SharedAnyToMany, SharedAnySet};
use crate::methods::{ViewAnyToMany, AnyToMany};
use crate::methods::{ViewSet, AnySet};

use crate::raw_junctions::many_to_many::RawManyToMany;

use std::collections::BTreeSet;

use crate::iterators::{ToManyKeysIterator, ToManyKeyValueIterator};

// == type ==
pub struct Bwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a ManyToMany<A, B> }
pub struct BwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_junctions) parent: &'a ManyToMany<A, B>, 
    pub(in crate::shared_junctions) key: B 
}

// == main impl ==
impl <'a, A: IdLike, B: IdLike> SharedAnyToMany<'a, B, A> for Bwd<'a, A, B> {
    type Multi = BwdSet<'a, A, B>;
    type Expunge = BTreeSet<A>;  

    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(B, Self::Multi)>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> Self::Multi { BwdSet { 
        parent: self.me, 
        key: b 
    } }
    fn contains_key(&self, b: B) -> bool { self.me.raw.borrow().bwd().contains_key(b) }

    fn len(&self) -> usize { self.me.raw.borrow().bwd().len() }  
    fn keys_len(&self) -> usize { self.me.raw.borrow().bwd().keys_len() }

    fn iter(&self) -> Self::Iter {
        BwdIterator::<'a, A, B> {
            iter: ToManyKeyValueIterator::new(
                self.me.raw.create_interior_set_range(),
                None, None
            )
        }
    }
    fn keys(&self) -> Self::Keys {
        BwdKeysIterator::<'a, A, B> { 
            iter: ToManyKeysIterator::new(self.me.raw.create_interior_map_range())
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
impl <'a, A: IdLike, B: IdLike> SharedAnySet<'a, A> for BwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { 
        let parent = self.parent.raw.borrow();
        let fetch = parent.bwd.get(self.key);
        fetch.contains(a) 
    }
    fn len(&self) -> usize { 
        let parent = self.parent.raw.borrow();
        let fetch = parent.bwd.get(self.key);
        fetch.len() 
    }

    fn iter(&self) -> Self::Iter {
        BwdSetIterator {
            iter: ToManyKeyValueIterator::new(
                self.parent.raw.create_interior_set_range(),
                Some((self.key, A::id_min_value())), 
                Some((self.key, A::id_max_value())),
            )
        }
    }

    fn insert(&self, a: A) -> Option<A> { self.parent.raw.borrow_mut().mut_bwd().get_mut(self.key).insert(a) }
    fn remove(&self, a: A) -> Option<A> { self.parent.raw.borrow_mut().mut_bwd().get_mut(self.key).remove(a) }
}

// == iterators ==
struct BwdIterator<'a, A: IdLike, B: IdLike> {
    iter: ToManyKeyValueIterator<'a, RawManyToMany<A, B>, B, A>,
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
    iter: ToManyKeysIterator<'a, RawManyToMany<A, B>, B>,
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
    iter: ToManyKeyValueIterator<'a, RawManyToMany<A, B>, B, A>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for BwdSetIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p| &p.bwd).map(|(_, v)| v)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for BwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.bwd).map(|(_, v)| v)
    }
}