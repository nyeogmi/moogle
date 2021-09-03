use std::hash::Hash;

use crate::methods::*;
use crate::IdLike;
use super::{MFwdSet, VFwdSet};

// == forward (set, mutable) ==
impl <'a, A: IdLike, B: IdLike> Extend<&'a B> for MFwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = &'a B>>(&mut self, iter: T) {
        for i in iter { self.insert(*i); }
    }
}

impl <'a, A: IdLike, B: IdLike> Extend<B> for MFwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = B>>(&mut self, iter: T) {
        for i in iter { self.insert(i); }
    }
}

impl <'a, A: IdLike, B: IdLike> Hash for MFwdSet<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl <'a, A: IdLike, B: IdLike> Ord for MFwdSet<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialEq<MFwdSet<'a, A, B>> for MFwdSet<'a, A, B> {
    fn eq(&self, other: &MFwdSet<'a, A, B>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialOrd<MFwdSet<'a, A, B>> for MFwdSet<'a, A, B> {
    fn partial_cmp(&self, other: &MFwdSet<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a, A: IdLike, B: IdLike> Eq for MFwdSet<'a, A, B> {

}

// == forward (set) ==
impl <'a, A: IdLike, B: IdLike> Hash for VFwdSet<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl <'a, A: IdLike, B: IdLike> Ord for VFwdSet<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialEq<VFwdSet<'a, A, B>> for VFwdSet<'a, A, B> {
    fn eq(&self, other: &VFwdSet<'a, A, B>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialOrd<VFwdSet<'a, A, B>> for VFwdSet<'a, A, B> {
    fn partial_cmp(&self, other: &VFwdSet<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a, A: IdLike, B: IdLike> Eq for VFwdSet<'a, A, B> {

}