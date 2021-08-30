use crate::id::IdLike;

use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, AnySet, EvictSet};

use crate::internal_structures::{ToSet, VSet, MSet};

use std::collections::BTreeSet;

// == Data structure ==
pub struct RawToSet<A: IdLike, B: IdLike> {
    pub(crate) fwd: ToSet<A, B>,
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> RawToSet<A, B> {
    pub fn new() -> RawToSet<A, B> {
        RawToSet { fwd: ToSet::new() }
    }
}

// == More structs ==
pub struct MFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawToSet<A, B>);
pub struct MFwdSet<'a, A: IdLike, B: IdLike>(pub(crate) MSet<'a, A, B>);

pub struct VFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawToSet<A, B>);
pub struct VFwdSet<'a, A: IdLike, B: IdLike>(pub(crate) VSet<'a, A, B>);

// == Accessors ==
impl<A: IdLike, B: IdLike> RawToSet<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
} 

// == Forward ==
impl<'a, A: IdLike, B: IdLike> AnyToSet<'a, A, B> for MFwd<'a, A, B> {
    type MMulti = MFwdSet<'a, A, B>;
    type MExpunge = BTreeSet<B>;

    fn get_mut(&'a mut self, a: A) -> MFwdSet<'a, A, B> {
        MFwdSet(self.0.fwd.get_mut(a))
    }

    fn insert(&mut self, a: A, b: B) -> Option<B> {
        self.0.fwd.insert(a.clone(), b.clone(), move |_, _| {})
     }

    fn expunge(&mut self, a: A) -> BTreeSet<B> { 
        self.0.fwd.expunge(a, move |_, _| {})
    }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToSet<'a, A, B> for MFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, A, B>;
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(A, Self::VMulti)>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&'a self, a: A) -> VFwdSet<'a, A, B> { VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
    fn keys_len(&self) -> usize { self.0.fwd.keys_len() }

    fn contains(&self, a: A, b: B) -> bool { self.0.fwd.get(a).contains(b) }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.fwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToSet<'a, A, B> for VFwd<'a, A, B> {
    type VMulti = VFwdSet<'a, A, B>;
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(A, Self::VMulti)>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> VFwdSet<'a, A, B> { VFwdSet(self.0.fwd.get(a)) }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }
    fn keys_len(&self) -> usize { self.0.fwd.keys_len() }

    fn contains(&self, a: A, b: B) -> bool { self.0.fwd.get(a).contains(b) }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.fwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

// == Forward (sets) ==
impl<'a, A: IdLike, B: IdLike> AnySet<'a, B> for MFwdSet<'a, A, B> {
    fn insert(&mut self, b: B) -> Option<B> { 
        self.0.insert(b.clone(), move |_, _| {})
    }
    fn remove(&mut self, b: B) -> Option<B> { 
        self.0.remove(b, move |_, _| {})
    }
}

impl<'a, A: IdLike, B: IdLike> ViewSet<'a, B> for MFwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=B>;

    fn contains(&self, b: B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}

impl<'a, A: IdLike, B: IdLike> ViewSet<'a, B> for VFwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=B>;

    fn contains(&self, b: B) -> bool { self.0.contains(b) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}