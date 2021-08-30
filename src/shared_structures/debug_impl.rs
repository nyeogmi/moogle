use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::methods::ViewAnyToSet;

use crate::IdLike;

// == set ==
impl<A: Debug+IdLike> Debug for super::set::Set<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<'a, A: Debug+IdLike> Debug for super::set::Fwd<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fmt(f) }
}

// == to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_one::ToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_one::Fwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fmt(f) }
}

// == to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_set::ToSet<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_set::Fwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fmt(f) }
}

impl<'a, A: IdLike, B: Debug+IdLike> Debug for super::to_set::FwdSet<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { 
        let owner = self.parent.raw.borrow();
        owner.fwd().get(self.key).fmt(f) 
    }
}