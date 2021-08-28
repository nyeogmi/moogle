use super::SetToOne;

use crate::methods::SharedAnyToOne;
use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::raw_junctions::set_to_one::RawSetToOne;

use crate::iterators::{FlatIterator};

use crate::id::IdLike;

// == type ==
pub struct Fwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a SetToOne<A, B> }

impl <'a, A: IdLike, B: IdLike> SharedAnyToOne<'a, A, B> for Fwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.me.raw.borrow().fwd().get(a) }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }
    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }

    fn iter(&self) -> Self::Iter {
        FwdFlatIterator::<'a, A, B> {
            iter: FlatIterator::new(self.me.raw.create_interior_map_range()),
        }
    }
    fn keys(&self) -> Self::Keys { self.iter().map(|(k, _)| k) }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == iterators ==
struct FwdFlatIterator<'a, A: IdLike, B: IdLike> {
    iter: FlatIterator<'a, RawSetToOne<A, B>, A, B>,

}
impl<'a, A: IdLike, B: IdLike> Iterator for FwdFlatIterator<'a, A, B> {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdFlatIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}