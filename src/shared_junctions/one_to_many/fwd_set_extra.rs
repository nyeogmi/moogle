use crate::methods::*;
use crate::IdLike;
use super::FwdSet;

// == forward (set, mutable) ==
impl <'a, A: IdLike, B: IdLike> SharedExtend<&'a B> for FwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = &'a B>>(&self, iter: T) {
        for i in iter { self.insert(*i); }
    }
}

impl <'a, A: IdLike, B: IdLike> SharedExtend<B> for FwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = B>>(&self, iter: T) {
        for i in iter { self.insert(i); }
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a FwdSet<'a, A, B> {
    type Item = B;

    type IntoIter = impl DoubleEndedIterator<Item=B>;

    fn into_iter(self) -> Self::IntoIter {
        self.parent.fwd().get(self.key).iter()
    }
}