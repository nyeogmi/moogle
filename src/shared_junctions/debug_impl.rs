use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::id::IdLike;

use crate::methods::ViewAnyToMany;


// == one-to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::OneToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

// == one-to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::OneToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: IdLike, B: Debug+IdLike> Debug for super::one_to_many::FwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().fwd().get(self.key).fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

// == set-to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::ManyToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

impl<A: Debug+IdLike, B: IdLike> Debug for super::many_to_one::BwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().bwd().get(self.key).fmt(f) }
}

// == set-to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::ManyToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: IdLike, B: IdLike+Debug> Debug for super::many_to_many::FwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().fwd().get(self.key).fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

impl<A: IdLike+Debug, B: IdLike> Debug for super::many_to_many::BwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().bwd().get(self.key).fmt(f) }
}