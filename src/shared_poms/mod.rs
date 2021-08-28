use crate::Id;

use crate::moogcell::MoogCell;

use crate::raw_poms::RawPom;

use crate::iterators::BTreeMapIterator;

pub struct Pom<T: 'static> { 
    raw: MoogCell<RawPom<T>>,
}

impl<T: 'static> Pom<T> {
    pub fn new() -> Self {
        Pom { raw: MoogCell::new(RawPom::new()) }
    }

    // Note: highlight in the documentation that this is a good idea if you intend to mutate it, 
    // since the iterable APIs aren't great
    pub fn raw(&mut self) -> &mut RawPom<T> { self.raw.get_mut() }

    pub fn insert(&self, t: T) -> Id<T> { self.raw.borrow_mut().insert(t) }
    pub fn remove(&self, k: Id<T>) -> Option<T> { self.raw.borrow_mut().remove(k) }
    pub fn transact(&self, k: Id<T>, f: impl FnOnce(Option<&T>)) { self.raw.borrow_mut().transact(k, f) }
    pub fn transact_mut(&self, k: Id<T>, f: impl FnOnce(Option<&mut T>)) { self.raw.borrow_mut().transact_mut(k, f) }

    // get() is &mut because people can wreak a lot of havoc with just a & and this struct
    pub fn get(&mut self, k: Id<T>) -> Option<&T> { self.raw.get_exclusive().get(k) }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut T> { self.raw.get_mut().get_mut(k) }
    pub fn contains_key(&self, k: Id<T>) -> bool { self.raw.borrow().contains_key(k) }
    pub fn len(&self) -> usize { self.raw.borrow().len() }

    // can't neatly provide iter_mut or values_mut because they both would require a moogcell borrow
    pub fn iter<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(Id<T>, &'a T)> {
        PomIterator {
            iter: BTreeMapIterator::new(self.raw.create_interior_btreemap_range())
        }
    }
    pub fn keys<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=Id<T>> {
        self.iter().map(move |(k, _)| k)
    }
    pub fn values<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=&'a T> {
        self.iter().map(move |(_, v)| v)
    }
}

struct PomIterator<'a, T: 'static> {
    iter: BTreeMapIterator<'a, RawPom<T>, Id<T>, T>,
}

impl<'a, T: 'static> Iterator for PomIterator<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> std::option::Option<Self::Item> { self.iter.next(|p| &p.members) }
}

impl <'a, T: 'static> DoubleEndedIterator for PomIterator<'a, T> {
    fn next_back(&mut self) -> std::option::Option<Self::Item> { self.iter.next_back(|p| &p.members) }
}