use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::keybound::Id;

use crate::methods::ViewAnyToSet;


// == one-to-one ==
impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_one::OneToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_one::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_one::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

// == one-to-set ==
impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_set::OneToSet<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_set::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Id, B: Debug+Id> Debug for super::one_to_set::FwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().fwd().get(self.key).fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::one_to_set::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

// == set-to-one ==
impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_one::SetToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_one::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_one::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

impl<A: Debug+Id, B: Id> Debug for super::set_to_one::BwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().bwd().get(self.key).fmt(f) }
}

// == set-to-set ==
impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_set::SetToSet<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.raw.borrow().fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_set::Fwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().fwd().fmt(f) }
}

impl<A: Id, B: Id+Debug> Debug for super::set_to_set::FwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().fwd().get(self.key).fmt(f) }
}

impl<A: Debug+Id, B: Debug+Id> Debug for super::set_to_set::Bwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.me.raw.borrow().bwd().fmt(f) }
}

impl<A: Id+Debug, B: Id> Debug for super::set_to_set::BwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.parent.raw.borrow().bwd().get(self.key).fmt(f) }
}