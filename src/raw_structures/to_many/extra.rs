use std::iter::FromIterator;

use super::{RawToMany, MFwd, VFwd};

use crate::IdLike;
use crate::methods::*;

impl<A: IdLike, B: IdLike> Default for RawToMany<A, B> {
    fn default() -> Self { Self::new() }
}

impl<'a, A: IdLike, B: IdLike> Extend<(&'a A, &'a B)> for RawToMany<A, B> {
    fn extend<T: IntoIterator<Item = (&'a A, &'a B)>>(&mut self, iter: T) {
        for (a, b) in iter {
            self.mut_fwd().insert(*a, *b);
        }
    }
}

impl<A: IdLike, B: IdLike> Extend<(A, B)> for RawToMany<A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&mut self, iter: T) {
        for (a, b) in iter {
            self.mut_fwd().insert(a, b);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> FromIterator<(&'a A, &'a B)> for RawToMany<A, B> {
    fn from_iter<T: IntoIterator<Item = (&'a A, &'a B)>>(iter: T) -> Self {
        let mut xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> FromIterator<(A, B)> for RawToMany<A, B> {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        let mut xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> IntoIterator for RawToMany<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fwd.into_iter()
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a RawToMany<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fwd.iter()
    }
}

// == Forward (mutable) ==
impl<'a, A: IdLike, B: IdLike> Extend<(&'a A, &'a B)> for MFwd<'a, A, B> {
    fn extend<T: IntoIterator<Item=(&'a A, &'a B)>>(&mut self, iter: T) {
        for (a, b) in iter { self.insert(*a, *b); }
    }
}

impl<'a, A: IdLike, B: IdLike> Extend<(A, B)> for MFwd<'a, A, B> {
    fn extend<T: IntoIterator<Item=(A, B)>>(&mut self, iter: T) {
        for (a, b) in iter { self.insert(a, b); }
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a MFwd<'a, A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// == Forward ==
impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a VFwd<'a, A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}