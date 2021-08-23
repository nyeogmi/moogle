use crate::keybound::Id;

use crate::methods::{ViewMultiMapLike, MultiMapLike, ViewMapLike, MapLike};
use crate::methods::{ViewSetLike, SetLike, EvictSetLike};

use crate::structures::{ToOne};
use crate::structures::{ToSet, VSet, MSet};

use std::collections::BTreeSet;

// == Data structure ==
pub struct SetToOne<A: Id, B: Id> {
    pub(crate) fwd: ToOne<A, B>,
    pub(crate) bwd: ToSet<B, A>,
}

// == Constructor et al ==
impl<A: Id, B: Id> SetToOne<A, B> {
    pub fn new() -> SetToOne<A, B> {
        SetToOne { fwd: ToOne::new(), bwd: ToSet::new() }
    }
}

// == More structs ==
pub struct MFwd<'a, A: Id, B: Id>(pub(crate) &'a mut SetToOne<A, B>);
pub struct MBwd<'a, A: Id, B: Id>(pub(crate) &'a mut SetToOne<A, B>);
pub struct MBwdSet<'a, A: Id, B: Id>(pub(crate) MSet<'a, B, A>, &'a mut ToOne<A, B>);

pub struct VFwd<'a, A: Id, B: Id>(pub(crate) &'a SetToOne<A, B>);
pub struct VBwd<'a, A: Id, B: Id>(pub(crate) &'a SetToOne<A, B>);
pub struct VBwdSet<'a, A: Id, B: Id>(pub(crate) VSet<'a, B, A>);

// == Accessors ==
impl<A: Id, B: Id> SetToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: Id, B: Id> SetToOne<A, B> {
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
    pub fn mut_bwd(&mut self) -> MBwd<A, B> { MBwd(self) }
} 

// == Forward ==
impl<'a, A: Id, B: Id> MapLike<'a, A, B> for MFwd<'a, A, B> {
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

impl<'a, A: Id, B: Id> ViewMapLike<'a, A, B> for MFwd<'a, A, B> {
    type Iter = impl 'a+Iterator<Item=(A, B)>;
    type Keys = impl 'a+Iterator<Item=A>;
    type Values = impl 'a+Iterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.fwd.values() }
}

impl<'a, A: Id, B: Id> ViewMapLike<'a, A, B> for VFwd<'a, A, B> {
    type Iter = impl 'a+Iterator<Item=(A, B)>;
    type Keys = impl 'a+Iterator<Item=A>;
    type Values = impl 'a+Iterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.0.fwd.get(a).as_option() }
    fn contains_key(&self, a: A) -> bool { self.0.fwd.contains_key(a) }
    fn len(&self) -> usize { self.0.fwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.fwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.fwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.fwd.values() }
}

// == Backward ==
impl<'a, A: Id, B: Id> MultiMapLike<'a, B, A> for MBwd<'a, A, B> {
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

impl<'a, A: Id, B: Id> ViewMultiMapLike<'a, B, A> for MBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A, B>;
    type Iter = impl 'a+Iterator<Item=(B, A)>;
    type Keys = impl 'a+Iterator<Item=B>;
    type Sets = impl 'a+Iterator<Item=(B, Self::VMulti)>;
    type Values = impl 'a+Iterator<Item=A>;

    fn get(&self, b: B) -> VBwdSet<'_, A, B> { VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.bwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

impl<'a, A: Id, B: Id> ViewMultiMapLike<'a, B, A> for VBwd<'a, A, B> {
    type VMulti = VBwdSet<'a, A, B>;
    type Iter = impl 'a+Iterator<Item=(B, A)>;
    type Keys = impl 'a+Iterator<Item=B>;
    type Sets = impl 'a+Iterator<Item=(B, Self::VMulti)>;
    type Values = impl 'a+Iterator<Item=A>;

    fn get(&self, b: B) -> VBwdSet<'_, A, B> { VBwdSet(self.0.bwd.get(b)) }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn sets(&'a self) -> Self::Sets { self.0.bwd.keys().map(move |k| (k, self.get(k))) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }
}

// == Backward (sets) ==
impl<'a, A: Id, B: Id> SetLike<'a, A> for MBwdSet<'a, A, B> {
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

impl<'a, A: Id, B: Id> ViewSetLike<'a, A> for MBwdSet<'a, A, B> {
    type Iter = impl 'a+Iterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}

impl<'a, A: Id, B: Id> ViewSetLike<'a, A> for VBwdSet<'a, A, B> {
    type Iter = impl 'a+Iterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.contains(a) }
    fn len(&self) -> usize { self.0.len() }

    fn iter(&'a self) -> Self::Iter { self.0.iter() }
}