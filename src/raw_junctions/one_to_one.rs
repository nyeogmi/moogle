use crate::id::IdLike;

use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::internal_structures::{ToOne};

// == Data structure ==
pub struct RawOneToOne<A: IdLike, B: IdLike> {
    pub(crate) fwd: ToOne<A, B>,
    pub(crate) bwd: ToOne<B, A>,
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> RawOneToOne<A, B> {
    pub fn new() -> RawOneToOne<A, B> {
        RawOneToOne { fwd: ToOne::new(), bwd: ToOne::new() }
    }
}

// == More structs ==
pub struct MFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawOneToOne<A, B>);
pub struct MBwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawOneToOne<A, B>);

pub struct VFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawOneToOne<A, B>);
pub struct VBwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawOneToOne<A, B>);

// == Accessors ==
impl<A: IdLike, B: IdLike> RawOneToOne<A, B> {
    pub fn fwd(&self) -> VFwd<A, B> { VFwd(self) }
    pub fn bwd(&self) -> VBwd<A, B> { VBwd(self) }
} 

impl<A: IdLike, B: IdLike> RawOneToOne<A, B> {
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
impl<'a, A: IdLike, B: IdLike> AnyToOne<'a, B, A> for MBwd<'a, A, B> {
    fn insert(&mut self, b: B, a: A) -> Option<A> {
        let fwd = &mut self.0.fwd;
        let result = self.0.bwd.insert(b.clone(), a.clone(), move |k, v| { fwd.remove(v, k, |_, _|{}); });

        let bwd = &mut self.0.bwd;
        self.0.fwd.insert(a, b, move |k, v| { bwd.remove(v, k, |_, _| {}); });
        result
     }

    fn expunge(&mut self, b: B) -> Option<A> { 
        let fwd = &mut self.0.fwd;
        self.0.bwd.expunge(b, move |k, v| { fwd.remove(v, k, |_, _|{}); })
    }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToOne<'a, B, A> for MBwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> Option<A> { self.0.bwd.get(b).as_option() }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.bwd.values() }
}

impl<'a, A: IdLike, B: IdLike> ViewAnyToOne<'a, B, A> for VBwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> Option<A> { self.0.bwd.get(b).as_option() }
    fn contains_key(&self, b: B) -> bool { self.0.bwd.contains_key(b) }
    fn len(&self) -> usize { self.0.bwd.len() }

    fn iter(&'a self) -> Self::Iter { self.0.bwd.iter() }
    fn keys(&'a self) -> Self::Keys { self.0.bwd.keys() }
    fn values(&'a self) -> Self::Values { self.0.bwd.values() }
}

// == TODO: Re-export Set views of VFwd et al ==