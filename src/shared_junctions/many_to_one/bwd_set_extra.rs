use crate::methods::*;
use crate::IdLike;
use super::BwdSet;

// == forward (set, mutable) ==
impl <'a, A: IdLike, B: IdLike> SharedExtend<&'a A> for BwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = &'a A>>(&self, iter: T) {
        for i in iter { self.insert(*i); }
    }
}

impl <'a, A: IdLike, B: IdLike> SharedExtend<A> for BwdSet<'a, A, B> {
    fn extend<T: IntoIterator<Item = A>>(&self, iter: T) {
        for i in iter { self.insert(i); }
    }
}

impl<'a, A: IdLike, B: IdLike> IntoIterator for &'a BwdSet<'a, A, B> {
    type Item = A;

    type IntoIter = impl DoubleEndedIterator<Item=A>;

    fn into_iter(self) -> Self::IntoIter {
        self.parent.bwd().get(self.key).iter()
    }
}