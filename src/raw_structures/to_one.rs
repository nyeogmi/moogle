use crate::id::IdLike;

use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::internal_structures::{ToOne};

// == Data structure ==
pub struct RawToOne<A: IdLike, B: IdLike> {
    pub(crate) fwd: ToOne<A, B>,
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> RawToOne<A, B> {
    pub fn new() -> RawToOne<A, B> {
        RawToOne { fwd: ToOne::new() }
    }
}

// == Private secret helper function ==
impl<A: IdLike, B: IdLike> RawToOne<A, B> {
    pub(crate) fn internal_keys<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=A> {
        self.fwd.keys()
    }
}

// == More structs ==
pub struct MFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a mut RawToOne<A, B>);

pub struct VFwd<'a, A: IdLike, B: IdLike>(pub(crate) &'a RawToOne<A, B>);

// == Accessors ==
impl<A: IdLike, B: IdLike> RawToOne<A, B> {
    pub fn fwd<'a>(&'a self) -> VFwd<'a, A, B> { VFwd(self) }
    pub fn mut_fwd(&mut self) -> MFwd<A, B> { MFwd(self) }
} 

// == Forward ==
impl<'a, A: IdLike, B: IdLike> AnyToOne<'a, A, B> for MFwd<'a, A, B> {
    fn insert(&mut self, a: A, b: B) -> Option<B> {
        self.0.fwd.insert(a, b, move |_, _| ()) 
     }

    fn expunge(&mut self, a: A) -> Option<B> { 
        self.0.fwd.expunge(a, move |_, _| ())
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