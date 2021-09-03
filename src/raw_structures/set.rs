use crate::id::IdLike;

use crate::methods::{ViewAnyToOne, AnyToOne};
use crate::methods::{ViewSet, AnySet};

use super::RawToOne;

// == Data structure ==
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct RawSet<A: IdLike> {
    pub(crate) underlying: RawToOne<A, ()>
}

// == Constructor et al ==
impl<A: IdLike> RawSet<A> {
    pub fn new() -> RawSet<A> {
        RawSet { underlying: RawToOne::new() }
    }
}

// == More structs ==
pub struct MFwd<'a, A: IdLike>(pub(crate) &'a mut RawSet<A>);

pub struct VFwd<'a, A: IdLike>(pub(crate) &'a RawSet<A>);

// == Accessors ==
impl<A: IdLike> RawSet<A> {
    pub fn fwd(&self) -> VFwd<A> { VFwd(self) }
    pub fn mut_fwd(&mut self) -> MFwd<A> { MFwd(self) }
} 

// == Forward ==
impl<'a, A: IdLike> AnySet<'a, A> for MFwd<'a, A> {
    fn insert(&mut self, a: A) -> Option<A> {
        self.0.underlying.mut_fwd().insert(a, ()).map(|_| a)
     }

    fn remove(&mut self, a: A) -> Option<A> {
        self.0.underlying.mut_fwd().expunge(a).map(|_| a)
    }
}

impl<'a, A: IdLike> ViewSet<'a, A> for MFwd<'a, A> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.underlying.fwd().contains_key(a) }
    fn len(&self) -> usize { self.0.underlying.fwd().len() }

    fn iter(&'a self) -> Self::Iter { self.0.underlying.internal_keys() }
}

impl<'a, A: IdLike> ViewSet<'a, A> for VFwd<'a, A> {
    type Iter = impl 'a+DoubleEndedIterator<Item=A>;

    fn contains(&self, a: A) -> bool { self.0.underlying.fwd().contains_key(a) }
    fn len(&self) -> usize { self.0.underlying.fwd().len() }

    fn iter(&'a self) -> Self::Iter { self.0.underlying.internal_keys() }
}