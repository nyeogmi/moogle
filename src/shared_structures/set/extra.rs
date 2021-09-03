use std::iter::FromIterator;

use super::{Set, Fwd};

use crate::IdLike;
use crate::methods::*;
use crate::moogcell::MoogCell;

// == Base type ==
impl<A: IdLike> Clone for Set<A> {
    fn clone(&self) -> Self {
        Self { raw: MoogCell::new(self.raw.borrow().clone()) }
    }
}

impl<A: IdLike> Default for Set<A> {
    fn default() -> Self { Self::new() }
}

impl<'a, A: IdLike> SharedExtend<&'a A> for Set<A> {
    fn extend<T: IntoIterator<Item=&'a A>>(&self, iter: T) {
        for a in iter {
            self.fwd().insert(*a);
        }
    }
}

impl<A: IdLike> SharedExtend<A> for Set<A> {
    fn extend<T: IntoIterator<Item = A>>(&self, iter: T) {
        for a in iter {
            self.fwd().insert(a);
        }
    }
}

impl<'a, A: IdLike> FromIterator<&'a A> for Set<A> {
    fn from_iter<T: IntoIterator<Item = &'a A>>(iter: T) -> Self {
        let xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike> FromIterator<A> for Set<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let xs = Self::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike> IntoIterator for Set<A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_inner().into_iter()
    }
}

impl<'a, A: IdLike> IntoIterator for &'a Set<A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fwd().iter()
    }
}

// == Forward ==
impl<'a, A: IdLike> SharedExtend<&'a A> for Fwd<'a, A> {
    fn extend<T: IntoIterator<Item=&'a A>>(&self, iter: T) {
        for a in iter { self.insert(*a); }
    }
}

impl<'a, A: IdLike> SharedExtend<A> for Fwd<'a, A> {
    fn extend<T: IntoIterator<Item = A>>(&self, iter: T) {
        for a in iter { self.insert(a); }
    }
}

impl<'a, A: IdLike> IntoIterator for &'a Fwd<'a, A> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}