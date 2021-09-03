mod extra;
mod bwd_set_extra;

use crate::id::IdLike;

use crate::methods::{ViewAnyToMany, AnyToMany, ViewAnyToOne, AnyToOne};
use crate::methods::{ViewSet, AnySet, EvictSet};

use crate::internal_structures::{ToOne};
use crate::internal_structures::{ToMany, VSet, MSet};

use std::collections::BTreeSet;

// == Data structure ==
#[derive(Clone)]
pub struct RawManyToOne<A: IdLike, B: IdLike> {
    pub(crate) fwd: ToOne<A, B>,
    pub(crate) bwd: ToMany<B, A>,
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> RawManyToOne<A, B> {
    pub fn new() -> RawManyToOne<A, B> {
        RawManyToOne { fwd: ToOne::new(), bwd: ToMany::new() }
    }
}

// == More structs ==
pub struct MFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawManyToOne<A, B>);
pub struct MBwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawManyToOne<A, B>);
pub struct MBwdSet<'a, A: IdLike, B: IdLike>(pub(crate) MSet<'a, B, A>, &'a mut ToOne<A, B>);

pub struct VFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawManyToOne<A, B>);
pub struct VBwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawManyToOne<A, B>);
pub struct VBwdSet<'a, A: IdLike, B: IdLike>(pub(crate) VSet<'a, B, A>);

// == Accessors ==
impl<A: IdLike, B: IdLike> RawManyToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: IdLike, B: IdLike> RawManyToOne<A, B> {
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
    pub fn mut_bwd(&mut self) -> MBwd<A, B> { MBwd(self) }
} 

// == Forward ==
impl<'a, A: IdLike, B: IdLike> AnyToOne<'a, A, B> for MFwd<'a, A, B> {
    fn insert(&mut self, a: A, b: B) -> Option<B> {
        let bwd = &mut self.0.bwd;
        let result = self.0.fwd.insert(a.clone(), b.clone(), move |k, v| { bwd.remove(v, k, |_, _|{}); });

        let fwd = &mut self.0.fwd;
        self.0.bwd.insert(b, a, move |k, v| { fwd.remove(v, k, |_, _| {}); });

        result
     }

    fn expunge(&mut self, a: A) -> Option<B> { 
        let bwd = &mut self.0.bwd;
        self.0.fwd.expunge(a, move |k, v| { bwd.remove(v, k, |_, _|{}); })
    }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToOne<'a, A, B> for MFwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.fwd.values() }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToOne<'a, A, B> for VFwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.fwd.values() }
}

// == Backward ==
impl<'a, A: IdLike, B: IdLike> AnyToMany<'a, B, A> for MBwd<'a, A, B> {
    type MMulti = MBwdSet<'a, A, B>;
    type MExpunge = BTreeSet<A>;

    fn get_mut(&'a mut self, b: B) -> MBwdSet<'a, A, B> {
        MBwdSet(self.0.bwd.get_mut(b), &mut self.0.fwd)
    }

    fn insert(&mut self, b: B, a: A) -> Option<A> {
        let fwd = &mut self.0.fwd;
        let result = self.0.bwd.insert(b.clone(), a.clone(), move |k, v| { fwd.remove(v, k, |_, _|{}); });

        let bwd = &mut self.0.bwd;
        self.0.fwd.insert(a, b, move |k, v| { bwd.remove(v, k, |_, _| {}); });
        result
     }

    fn expunge(&mut self, b: B) -> BTreeSet<A> { 
        let fwd = &mut self.0.fwd;
        self.0.bwd.expunge(b, move |k, v| { fwd.remove(v, k, |_, _| {}); })
    }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToMany<'a, B, A> for MBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A, B>;
    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(B, Self::VMulti)>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&'a self, b: B) -> VBwdSet<'a, A, B> { VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }
    fn keys_len(&self) -> usize { self.0.bwd.keys_len() }

    fn contains(&self, b: B, a: A) -> bool { self.0.bwd.get(b).contains(a) }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.bwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToMany<'a, B, A> for VBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A, B>;
    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(B, Self::VMulti)>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> VBwdSet<'a, A, B> { VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }

    fn len(&self) -> usize { self.0.bwd.len() }
    fn keys_len(&self) -> usize { self.0.bwd.keys_len() }

    fn contains(&self, b: B, a: A) -> bool { self.0.bwd.get(b).contains(a) }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.bwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

// == Backward (sets) ==
impl<'a, A: IdLike, B: IdLike> AnySet<'a, A> for MBwdSet<'a, A, B> {
    fn insert(&mut self, a: A) -> Option<A> { 
        let alt = &mut self.1;
        let result = self.0.insert(a.clone(), move |k, v| { alt.remove(v, k, |_, _|{}); });

        let key = self.0.key().clone();
        let stt = &mut self.0;

        self.1.insert(a, key, move |k, _| { stt.remove(k, |_, _| {}); });
        result
    }
    fn remove(&mut self, a: A) -> Option<A> { 
        let opposite = &mut self.1;
        self.0.remove(a, move |k, v| { opposite.remove(v, k, |_, _|{}); }) 
    }
}

impl<'a, A: IdLike, B: IdLike> ViewSet<'a, A> for MBwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}

impl<'a, A: IdLike, B: IdLike> ViewSet<'a, A> for VBwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}