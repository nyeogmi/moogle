use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::methods::ViewAnyToMany;

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
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_many::ToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_many::Fwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fmt(f) }
}

impl<'a, A: IdLike, B: Debug+IdLike> Debug for super::to_many::FwdSet<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { 
        let owner = self.parent.raw.borrow();
        owner.fwd().get(self.key).fmt(f) 
    }
}