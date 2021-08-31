use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::id::IdLike;

use crate::internal_structures::{ToOne, ToMany};
use crate::methods::ViewSet;


fn to_one<A: Debug+IdLike, B: Debug+IdLike>(f: &mut Formatter<'_>, t: &ToOne<A, B>) -> fmt::Result {
    f.debug_map().entries(t.iter()).finish()
}

fn to_many<A: Debug+IdLike, B: Debug+IdLike>(f: &mut Formatter<'_>, t: &ToMany<A, B>) -> fmt::Result {
    f.debug_map().entries(t.sets()).finish()
}

fn set<'a, V: Debug+IdLike>(f: &mut Formatter<'_>, s: &'a impl ViewSet<'a, V>) -> fmt::Result {
    f.debug_set().entries(s.iter()).finish()
}

// == one-to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::RawOneToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::MFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::MBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.bwd) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::VFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd ) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_one::VBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.bwd) }
}

// == one-to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::RawOneToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::MFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<A: IdLike, B: Debug+IdLike> Debug for super::one_to_many::MFwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::MBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.bwd) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::VFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<A: IdLike, B: Debug+IdLike> Debug for super::one_to_many::VFwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::one_to_many::VBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.bwd) }
}

// == set-to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::RawManyToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::MFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::MBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.bwd) }
}

impl<A: Debug+IdLike, B: IdLike> Debug for super::many_to_one::MBwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::VFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_one::VBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.bwd) }
}

impl<A: Debug+IdLike, B: IdLike> Debug for super::many_to_one::VBwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

// == set-to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::RawManyToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { self.fwd().fmt(f) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::MFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<A: IdLike, B: IdLike+Debug> Debug for super::many_to_many::MFwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::MBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.bwd) }
}

impl<A: IdLike+Debug, B: IdLike> Debug for super::many_to_many::MBwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::VFwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<A: IdLike, B: IdLike+Debug> Debug for super::many_to_many::VFwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}

impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::many_to_many::VBwd<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.bwd) }
}

impl<A: IdLike+Debug, B: IdLike> Debug for super::many_to_many::VBwdSet<'_, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, &self.0) }
}