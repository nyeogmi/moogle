use super::SharedOneToOne;

use crate::methods::SharedAnyToOne;
use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::junctions::one_to_one::OneToOne;

use super::super::iterators::{FlatIterator};

use crate::keybound::Id;

// == type ==
pub struct Fwd<'a, A: Id, B: Id> { pub(super) me: &'a SharedOneToOne<A, B> }

impl <'a, A: Id, B: Id> SharedAnyToOne<'a, A, B> for Fwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.me.raw.borrow().fwd().get(a) }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }
    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }

    fn iter(&'a self) -> Self::Iter {
        FwdFlatIterator::<'a, A, B> {
            me: self.me,
            iter: FlatIterator::new(self.me.raw.create_interior_tree_range()),
        }
    }
    fn keys(&'a self) -> Self::Keys { self.iter().map(|(k, _)| k) }
    fn values(&'a self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == iterators ==
struct FwdFlatIterator<'a, A: Id, B: Id> {
    me: &'a SharedOneToOne<A, B>,
    iter: FlatIterator<OneToOne<A, B>, A, B>,

}
impl<'a, A: Id, B: Id> Iterator for FwdFlatIterator<'a, A, B> {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        self.iter.next(&self.me.raw, |p| &p.fwd)
    }
}

impl <'a, A: Id, B: Id> DoubleEndedIterator for FwdFlatIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(&self.me.raw, |p| &p.fwd)
    }
}