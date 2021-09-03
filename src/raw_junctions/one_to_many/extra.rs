// stock trait implementations
// trait list taken from https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
// not implementing Index, it requires a Ref-based interface
// not implementing IntoIterator here as it has complicated lifetime issues

use std::{iter::FromIterator};
use std::hash::Hash;

use crate::methods::*;
use crate::IdLike;
use super::{RawOneToMany, VFwd, MFwd, VBwd, MBwd};

type Me<A, B> = RawOneToMany<A, B>;

impl<A: IdLike, B: IdLike> Default for Me<A, B> {
    fn default() -> Self { Me::new() }
}

// TODO: Reexport trait methods for base type

// == base type ==
impl<'a, A: IdLike, B: IdLike> Extend<(&'a A, &'a B)> for Me<A, B> {
    fn extend<T: IntoIterator<Item = (&'a A, &'a B)>>(&mut self, iter: T) {
        self.mut_fwd().extend(iter)
    }
}

impl<A: IdLike, B: IdLike> Extend<(A, B)> for Me<A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&mut self, iter: T) {
        self.mut_fwd().extend(iter)
    }
}

impl<'a, A: IdLike, B: IdLike> FromIterator<(&'a A, &'a B)> for Me<A, B> {
    fn from_iter<T: IntoIterator<Item = (&'a A, &'a B)>>(iter: T) -> Self {
        let mut xs = Me::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> FromIterator<(A, B)> for Me<A, B> {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        let mut xs = Me::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> Hash for Me<A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fwd().hash(state);
    }
}

impl<A: IdLike, B: IdLike> IntoIterator for Me<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=(A, B)>;

    fn into_iter(self) -> Self::IntoIter {
        self.fwd.into_iter()
    }
}

// don't provide IntoIterator for & and &mut -- 
// we only provide it so it is _possible_ to consume into an iterator
// and you should really use the fwd() and bwd() accessors for that

impl<A: IdLike, B: IdLike> Ord for Me<A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fwd().iter().cmp(other.fwd().iter())
    }
}

impl<A: IdLike, B: IdLike> PartialEq<Me<A, B>> for Me<A, B> {
    fn eq(&self, other: &Me<A, B>) -> bool {
        self.fwd == other.fwd 
    }
}

impl<A: IdLike, B: IdLike> PartialOrd<Me<A, B>> for Me<A, B> {
    fn partial_cmp(&self, other: &Me<A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: IdLike, B: IdLike> Eq for Me<A, B> {

}

// == forward (mut) == 
impl<'a, 'b, A: IdLike, B: IdLike> Extend<(&'b A, &'b B)> for MFwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (&'b A, &'b B)>>(&mut self, iter: T) {
        for (a, b) in iter {
            self.insert(*a, *b);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> Extend<(A, B)> for MFwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&mut self, iter: T) {
        for (a, b) in iter {
            self.insert(a, b);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> Hash for MFwd<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl<'a, A: IdLike, B: IdLike> Ord for MFwd<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.fwd.cmp(&other.0.fwd)
    }
}

impl<'a, A: IdLike, B: IdLike> PartialEq<MFwd<'a, A, B>> for MFwd<'a, A, B> {
    fn eq(&self, other: &MFwd<'a, A, B>) -> bool {
        self.0.fwd == other.0.fwd
    }
}

impl<'a, A: IdLike, B: IdLike> PartialOrd<MFwd<'a, A, B>> for MFwd<'a, A, B> {
    fn partial_cmp(&self, other: &MFwd<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, A: IdLike, B: IdLike> Eq for MFwd<'a, A, B> {

}

// == forward (immut) == 
impl<'a, A: IdLike, B: IdLike> Hash for VFwd<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl<'a, A: IdLike, B: IdLike> Ord for VFwd<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.fwd.cmp(&other.0.fwd)
    }
}

impl<'a, A: IdLike, B: IdLike> PartialEq<VFwd<'a, A, B>> for VFwd<'a, A, B> {
    fn eq(&self, other: &VFwd<'a, A, B>) -> bool {
        self.0.fwd == other.0.fwd
    }
}

impl<'a, A: IdLike, B: IdLike> PartialOrd<VFwd<'a, A, B>> for VFwd<'a, A, B> {
    fn partial_cmp(&self, other: &VFwd<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, A: IdLike, B: IdLike> Eq for VFwd<'a, A, B> {

}

// == backward (mut) == 
impl<'a, 'b, A: IdLike, B: IdLike> Extend<(&'b B, &'b A)> for MBwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (&'b B, &'b A)>>(&mut self, iter: T) {
        for (b, a) in iter {
            self.insert(*b, *a);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> Extend<(B, A)> for MBwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (B, A)>>(&mut self, iter: T) {
        for (b, a) in iter {
            self.insert(b, a);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> Hash for MBwd<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl<'a, A: IdLike, B: IdLike> Ord for MBwd<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.bwd.cmp(&other.0.bwd)
    }
}

impl<'a, A: IdLike, B: IdLike> PartialEq<MBwd<'a, A, B>> for MBwd<'a, A, B> {
    fn eq(&self, other: &MBwd<'a, A, B>) -> bool {
        self.0.bwd == other.0.bwd
    }
}

impl<'a, A: IdLike, B: IdLike> PartialOrd<MBwd<'a, A, B>> for MBwd<'a, A, B> {
    fn partial_cmp(&self, other: &MBwd<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, A: IdLike, B: IdLike> Eq for MBwd<'a, A, B> {

}

// == backward (immut) == 
impl<'a, A: IdLike, B: IdLike> Hash for VBwd<'a, A, B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in self.iter() { i.hash(state) }
    }
}

impl<'a, A: IdLike, B: IdLike> Ord for VBwd<'a, A, B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.bwd.cmp(&other.0.bwd)
    }
}

impl<'a, A: IdLike, B: IdLike> PartialEq<VBwd<'a, A, B>> for VBwd<'a, A, B> {
    fn eq(&self, other: &VBwd<'a, A, B>) -> bool {
        self.0.bwd == other.0.bwd
    }
}

impl<'a, A: IdLike, B: IdLike> PartialOrd<VBwd<'a, A, B>> for VBwd<'a, A, B> {
    fn partial_cmp(&self, other: &VBwd<'a, A, B>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, A: IdLike, B: IdLike> Eq for VBwd<'a, A, B> {

}