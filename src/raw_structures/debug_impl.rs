use std::fmt::{Debug, Formatter};
use std::fmt;

use crate::IdLike;

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

// == set ==
impl<A: Debug+IdLike> Debug for super::set::RawSet<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { 
        f.debug_set().entries(self.fwd().iter()).finish()
    }
}

impl<'a, A: Debug+IdLike> Debug for super::set::MFwd<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { 
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<'a, A: Debug+IdLike> Debug for super::set::VFwd<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { 
        f.debug_set().entries(self.iter()).finish()
    }
}

// == to-one ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_one::RawToOne<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.fwd) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_one::MFwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_one::VFwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_one(f, &self.0.fwd) }
}

// == to-set ==
impl<A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_many::RawToMany<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.fwd) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_many::MFwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<'a, A: Debug+IdLike, B: Debug+IdLike> Debug for super::to_many::VFwd<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { to_many(f, &self.0.fwd) }
}

impl<'a, A: IdLike, B: Debug+IdLike> Debug for super::to_many::MFwdSet<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, self) }
}

impl<'a, A: IdLike, B: Debug+IdLike> Debug for super::to_many::VFwdSet<'a, A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { set(f, self) }
}