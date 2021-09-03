use std::iter::FromIterator;

use super::{RawSet, MFwd, VFwd};

use crate::IdLike;
use crate::methods::*;

// == Base type ==
impl<A: IdLike> Default for RawSet<A> {
    fn default() -> Self { Self::new() }
}

impl<'a, A: IdLike> Extend<&'a A> for RawSet<A> {
    fn extend<T: IntoIterator<Item=&'a A>>(&mut self, iter: T) {
        for a in iter {
            self.mut_fwd().insert(*a);
        }
    }
}

impl<A: IdLike> Extend<A> for RawSet<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for a in iter {
            self.mut_fwd().insert(a);
        }
    }
}

impl<'a, A: IdLike> FromIterator<&'a A> for RawSet<A> {
    fn from_iter<T: IntoIterator<Item = &'a A>>(iter: T) -> Self {
        let mut xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike> FromIterator<A> for RawSet<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike> IntoIterator for RawSet<A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.underlying.into_iter().map(|(k, _)| k)
    }
}

impl<'a, A: IdLike> IntoIterator for &'a RawSet<A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.underlying.fwd.iter().map(|(k, _)| k)
    }
}

// == Forward (mut) ==
impl<'a, A: IdLike> Extend<&'a A> for MFwd<'a, A> {
    fn extend<T: IntoIterator<Item=&'a A>>(&mut self, iter: T) {
        for a in iter { self.insert(*a); }
    }
}

impl<'a, A: IdLike> Extend<A> for MFwd<'a, A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for a in iter { self.insert(a); }
    }
}

impl<'a, A: IdLike> IntoIterator for &'a MFwd<'a, A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// == Forward ==
impl<'a, A: IdLike> IntoIterator for &'a VFwd<'a, A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}