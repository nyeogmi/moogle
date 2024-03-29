use super::OneToMany;

use crate::methods::SharedAnyToOne;
use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::raw_junctions::one_to_many::RawOneToMany;

use crate::iterators::BTreeMapIterator;

use crate::id::IdLike;

// == type ==
pub struct Bwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a OneToMany<A, B> }

impl <'a, A: IdLike, B: IdLike> SharedAnyToOne<'a, B, A> for Bwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(B, A)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=B>;
    type Values = impl 'a+DoubleEndedIterator<Item=A>;

    fn get(&self, b: B) -> Option<A> { self.me.raw.borrow().bwd().get(b) }
    fn contains_key(&self, b: B) -> bool { self.me.raw.borrow().bwd().contains_key(b) }
    fn len(&self) -> usize { self.me.raw.borrow().bwd().len() }

    fn iter(&self) -> Self::Iter {
        BwdFlatIterator::<'a, A, B> {
            iter: BTreeMapIterator::new(self.me.raw.create_interior_map_range()),
        }
    }
    fn keys(&self) -> Self::Keys { self.iter().map(|(k, _)| k) }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, b: B, a: A) -> Option<A> { self.me.raw.borrow_mut().mut_bwd().insert(b, a) }
    fn expunge(&self, b: B) -> Option<A> { self.me.raw.borrow_mut().mut_bwd().expunge(b) }
}

// == iterators ==
struct BwdFlatIterator<'a, A: IdLike, B: IdLike> {
    iter: BTreeMapIterator<'a, RawOneToMany<A, B>, B, A>,

}
impl<'a, A: IdLike, B: IdLike> Iterator for BwdFlatIterator<'a, A, B> {
    type Item = (B, A);

    fn next(&mut self) -> Option<(B, A)> {
        self.iter.next(|p| &p.bwd.0)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for BwdFlatIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.bwd.0)
    }
}