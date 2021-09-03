use crate::id::IdLike;

use crate::raw_structures::RawToOne;
use crate::moogcell::MoogCell;

use crate::methods::SharedAnyToOne;
use crate::methods::{ViewAnyToOne, AnyToOne};

use crate::iterators::BTreeMapIterator;

mod extra;

// == Data structure ==
pub struct ToOne<A: IdLike, B: IdLike> {
    pub(in crate::shared_structures) raw: MoogCell<RawToOne<A, B>>
}

// == Constructor et al ==
impl<A: IdLike, B: IdLike> ToOne<A, B> {
    pub fn new() -> ToOne<A, B> {
        ToOne { raw: MoogCell::new(RawToOne::new()) }
    }

    pub fn raw(&mut self) -> &mut RawToOne<A, B> { self.raw.get_mut() }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
}

// Shared fwd
pub struct Fwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_structures) me: &'a ToOne<A, B> }

impl <'a, A: IdLike, B: IdLike> SharedAnyToOne<'a, A, B> for Fwd<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Option<B> { self.me.raw.borrow().fwd().get(a) }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }
    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }

    fn iter(&self) -> Self::Iter {
        FwdFlatIterator::<'a, A, B> {
            iter: BTreeMapIterator::new(self.me.raw.create_interior_map_range()),
        }
    }
    fn keys(&self) -> Self::Keys { self.iter().map(|(k, _)| k) }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == iterators ==
struct FwdFlatIterator<'a, A: IdLike, B: IdLike> {
    iter: BTreeMapIterator<'a, RawToOne<A, B>, A, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdFlatIterator<'a, A, B> {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        self.iter.next(|p| &p.fwd.0)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdFlatIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd.0)
    }
}