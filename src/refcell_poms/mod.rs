use std::cell::RefCell;

use crate::{Id, Set, RawPom};

use crate::methods::*;

use self::cast::{cast_plain, cast_refcell};

mod cast;
mod debug_impl;

#[cfg(feature="serde1")]
mod serde_impl;

mod extra;

#[derive(Clone)]
pub struct RefCellPom<T: 'static> { 
    index: Set<Id<T>>,
    elements: RawPom<RefCell<T>>,
}

impl<T: 'static> RefCellPom<T> {
    pub fn new() -> Self {
        RefCellPom { 
            index: Set::new(),
            elements: RawPom::new(),
        }
    }

    #[cfg(feature="serde1")]  // only needs to exist for serde
    pub(crate) fn from_raw(elements: RawPom<RefCell<T>>) -> Self {
        let index = Set::new();
        for k in elements.keys() { index.fwd().insert(cast_plain(k)); }
        RefCellPom { index, elements }
    }

    pub fn insert(&mut self, t: RefCell<T>) -> Id<T> { 
        let id = cast_plain(self.elements.insert(t));
        self.index.fwd().insert(id);
        id
    }
    pub fn remove(&mut self, k: Id<T>) -> Option<RefCell<T>> { 
        self.index.fwd().remove(k);
        self.elements.remove(cast_refcell(k)) 
    }

    pub fn get(&self, k: Id<T>) -> Option<&RefCell<T>> { 
        self.elements.get(cast_refcell(k)) 
    }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut RefCell<T>> { 
        self.elements.get_mut(cast_refcell(k)) 
    }
    pub fn contains_key(&self, k: Id<T>) -> bool { 
        self.elements.contains_key(cast_refcell(k))
    }
    pub fn len(&self) -> usize { 
        self.elements.len() 
    }

    pub fn share<'a>(&'a mut self) -> (Index<'a, T>, Elements<'a, T>) {
        (Index(&self.index), Elements(&mut self.elements))
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item=(Id<T>, &RefCell<T>)> {
        self.elements.iter().map(|(k, v)| (cast_plain(k), v))
    }
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item=(Id<T>, &mut RefCell<T>)> {
        self.elements.iter_mut().map(|(k, v)| (cast_plain(k), v))
    }
    pub fn keys(&self) -> impl '_+DoubleEndedIterator<Item=Id<T>> {
        self.elements.keys().map(|k| cast_plain(k))
    }
    pub fn values(&mut self) -> impl DoubleEndedIterator<Item=&RefCell<T>> {
        self.iter().map(move |(_, v)| v)
    }
    pub fn values_mut(&mut self) -> impl DoubleEndedIterator<Item=&RefCell<T>> {
        self.iter().map(move |(_, v)| v)
    }
}

pub struct Index<'a, T: 'static> (&'a Set<Id<T>>);
pub struct Elements<'a, T: 'static> (&'a mut RawPom<RefCell<T>>);

impl<'a, T> Index<'a, T> {
    pub fn keys(&'a self) -> impl 'a+DoubleEndedIterator<Item=Id<T>> {
        self.0.fwd().iter()
    }
}

impl<'a, T> Elements<'a, T> {
    pub fn get(&self, k: Id<T>) -> Option<&RefCell<T>> { 
        self.0.get(cast_refcell(k))
    }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut RefCell<T>> { 
        self.0.get_mut(cast_refcell(k))
    }
    pub fn contains_key(&self, k: Id<T>) -> bool { 
        self.0.contains_key(cast_refcell(k))
    }
    pub fn len(&self) -> usize { 
        self.0.len() 
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item=(Id<T>, &RefCell<T>)> {
        self.0.iter().map(|(k, v)| (cast_plain(k), v))
    }
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item=(Id<T>, &mut RefCell<T>)> {
        self.0.iter_mut().map(|(k, v)| (cast_plain(k), v))
    }
    pub fn keys(&self) -> impl '_+DoubleEndedIterator<Item=Id<RefCell<T>>> {
        self.0.keys()
    }
    pub fn values(&self) -> impl DoubleEndedIterator<Item=&RefCell<T>> { 
        self.0.values() 
    }
    pub fn values_mut(&mut self) -> impl DoubleEndedIterator<Item=&mut RefCell<T>> { 
        self.0.values_mut() 
    }
}