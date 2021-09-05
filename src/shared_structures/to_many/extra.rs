use std::iter::FromIterator;

use super::{ToMany, Fwd};

use crate::IdLike;
use crate::methods::*;
use crate::moogcell::MoogCell;

impl<A: IdLike, B: IdLike> Clone for ToMany<A, B> {
    fn clone(&self) -> Self {
        Self { raw: MoogCell::new(self.raw.borrow().clone()) }
    }
}

impl<A: IdLike, B: IdLike> Default for ToMany<A, B> {
    fn default() -> Self { Self::new() }
}

impl<'a, A: IdLike, B: IdLike> SharedExtend<(&'a A, &'a B)> for ToMany<A, B> {
    fn extend<T: IntoIterator<Item = (&'a A, &'a B)>>(&self, iter: T) {
        for (a, b) in iter {
            self.fwd().insert(*a, *b);
        }
    }
}

impl<A: IdLike, B: IdLike> SharedExtend<(A, B)> for ToMany<A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&self, iter: T) {
        for (a, b) in iter {
            self.fwd().insert(a, b);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> FromIterator<(&'a A, &'a B)> for ToMany<A, B> {
    fn from_iter<T: IntoIterator<Item = (&'a A, &'a B)>>(iter: T) -> Self {
        let xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> FromIterator<(A, B)> for ToMany<A, B> {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        let xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> IntoIterator for ToMany<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_inner().into_iter()
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a ToMany<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fwd().iter()
    }
}

// == Forward (mutable) ==
impl<'a, A: IdLike, B: IdLike> Extend<(&'a A, &'a B)> for Fwd<'a, A, B> {
    fn extend<T: IntoIterator<Item=(&'a A, &'a B)>>(&mut self, iter: T) {
        for (a, b) in iter { self.insert(*a, *b); }
    }
}

impl<'a, A: IdLike, B: IdLike> Extend<(A, B)> for Fwd<'a, A, B> {
    fn extend<T: IntoIterator<Item=(A, B)>>(&mut self, iter: T) {
        for (a, b) in iter { self.insert(a, b); }
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a Fwd<'a, A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}