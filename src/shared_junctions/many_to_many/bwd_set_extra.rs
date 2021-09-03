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