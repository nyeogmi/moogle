// stock trait implementations
// trait list taken from https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
// not implementing Index, it requires a Ref-based interface
// not implementing IntoIterator here as it has complicated lifetime issues
// not implementing Eq, Hash and Ord, as it is not safe to rely on 
// those interfaces for types that support sharing

use std::{iter::FromIterator};

use crate::methods::*;
use crate::IdLike;
use super::{ManyToMany, Fwd, Bwd};

type Me<A, B> = ManyToMany<A, B>;

impl<A: IdLike, B: IdLike> Default for Me<A, B> {
    fn default() -> Self { Me::new() }
}

// == base type ==
impl<'a, A: IdLike, B: IdLike> SharedExtend<(&'a A, &'a B)> for Me<A, B> {
    fn extend<T: IntoIterator<Item = (&'a A, &'a B)>>(&self, iter: T) {
        self.fwd().extend(iter)
    }
}

impl<A: IdLike, B: IdLike> SharedExtend<(A, B)> for Me<A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&self, iter: T) {
        self.fwd().extend(iter)
    }
}

impl<'a, A: IdLike, B: IdLike> FromIterator<(&'a A, &'a B)> for Me<A, B> {
    fn from_iter<T: IntoIterator<Item = (&'a A, &'a B)>>(iter: T) -> Self {
        let xs = Me::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> FromIterator<(A, B)> for Me<A, B> {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        let xs = Me::new();
        xs.extend(iter);
        xs
    }
}

impl<A: IdLike, B: IdLike> IntoIterator for Me<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=(A, B)>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_inner().into_iter()
    }
}

// == forward == 
impl<'a, 'b, A: IdLike, B: IdLike> SharedExtend<(&'b A, &'b B)> for Fwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (&'b A, &'b B)>>(&self, iter: T) {
        for (a, b) in iter {
            self.insert(*a, *b);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> SharedExtend<(A, B)> for Fwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (A, B)>>(&self, iter: T) {
        for (a, b) in iter {
            self.insert(a, b);
        }
    }
}

// == backward == 
impl<'a, 'b, A: IdLike, B: IdLike> SharedExtend<(&'b B, &'b A)> for Bwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (&'b B, &'b A)>>(&self, iter: T) {
        for (b, a) in iter {
            self.insert(*b, *a);
        }
    }
}

impl<'a, A: IdLike, B: IdLike> SharedExtend<(B, A)> for Bwd<'a, A, B> {
    fn extend<T: IntoIterator<Item = (B, A)>>(&self, iter: T) {
        for (b, a) in iter {
            self.insert(b, a);
        }
    }
}