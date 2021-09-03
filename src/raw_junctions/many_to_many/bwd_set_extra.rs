use std::hash::Hash;

use crate::methods::*;
use crate::IdLike;
use super::{MBwdSet, VBwdSet};

// == forward (set, mutable) ==
impl <'a, A: IdLike, B: IdLike> Extend<&'a A> for MBwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = &'a A>>(&mut self, iter: T) {
        for i in iter { self.insert(*i); }
    }
}

impl <'a, A: IdLike, B: IdLike> Extend<A> for MBwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for i in iter { self.insert(i); }
    }
}

impl <'a, A: IdLike, B: IdLike> Hash for MBwdSet<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl <'a, A: IdLike, B: IdLike> Ord for MBwdSet<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialEq<MBwdSet<'a, A, B>> for MBwdSet<'a, A, B> {
    fn eq(&self, other: &MBwdSet<'a, A, B>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialOrd<MBwdSet<'a, A, B>> for MBwdSet<'a, A, B> {
    fn partial_cmp(&self, other: &MBwdSet<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a, A: IdLike, B: IdLike> Eq for MBwdSet<'a, A, B> {

}

// == forward (set) ==
impl <'a, A: IdLike, B: IdLike> Hash for VBwdSet<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl <'a, A: IdLike, B: IdLike> Ord for VBwdSet<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialEq<VBwdSet<'a, A, B>> for VBwdSet<'a, A, B> {
    fn eq(&self, other: &VBwdSet<'a, A, B>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl <'a, A: IdLike, B: IdLike> PartialOrd<VBwdSet<'a, A, B>> for VBwdSet<'a, A, B> {
    fn partial_cmp(&self, other: &VBwdSet<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a, A: IdLike, B: IdLike> Eq for VBwdSet<'a, A, B> {

}