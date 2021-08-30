use crate::{Id, Set, RawPom};

use crate::methods::*;

mod debug_impl;

pub struct Pom<T: 'static> { 
    index: Set<Id<T>>,
    elements: RawPom<T>,
}

impl<T: 'static> Pom<T> {
    pub fn new() -> Self {
        Pom { 
            index: Set::new(),
            elements: RawPom::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> Id<T> { 
        let id = self.elements.insert(t);
        self.index.fwd().insert(id);
        id
    }
    pub fn remove(&mut self, k: Id<T>) -> Option<T> { 
        self.index.fwd().remove(k);
        self.elements.remove(k) 
    }

    pub fn get(&self, k: Id<T>) -> Option<&T> { 
        self.elements.get(k) 
    }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut T> { 
        self.elements.get_mut(k) 
    }
    pub fn contains_key(&self, k: Id<T>) -> bool { 
        self.elements.contains_key(k)
    }
    pub fn len(&self) -> usize { 
        self.elements.len() 
    }

    pub fn share<'a>(&'a mut self) -> (Index<'a, T>, Elements<'a, T>) {
        (Index(&self.index), Elements(&mut self.elements))
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item=(Id<T>, &T)> {
        self.elements.iter()
    }
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item=(Id<T>, &mut T)> {
        self.elements.iter_mut()
    }
    pub fn keys(&self) -> impl '_+DoubleEndedIterator<Item=Id<T>> {
        self.elements.keys()
    }
    pub fn values(&mut self) -> impl DoubleEndedIterator<Item=&T> {
        self.iter().map(move |(_, v)| v)
    }
    pub fn values_mut(&mut self) -> impl DoubleEndedIterator<Item=&T> {
        self.iter().map(move |(_, v)| v)
    }
}

pub struct Index<'a, T: 'static> (&'a Set<Id<T>>);
pub struct Elements<'a, T: 'static> (&'a mut RawPom<T>);

impl<'a, T> Index<'a, T> {
    pub fn keys(&'a self) -> impl 'a+DoubleEndedIterator<Item=Id<T>> {
        self.0.fwd().iter()
    }
}

impl<'a, T> Elements<'a, T> {
    pub fn get(&self, k: Id<T>) -> Option<&T> { 
        self.0.get(k) 
    }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut T> { 
        self.0.get_mut(k) 
    }
    pub fn contains_key(&self, k: Id<T>) -> bool { 
        self.0.contains_key(k)
    }
    pub fn len(&self) -> usize { 
        self.0.len() 
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item=(Id<T>, &T)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item=(Id<T>, &mut T)> {
        self.0.iter_mut()
    }
    pub fn keys(&self) -> impl '_+DoubleEndedIterator<Item=Id<T>> {
        self.0.keys()
    }
    pub fn values(&self) -> impl DoubleEndedIterator<Item=&T> { 
        self.0.values() 
    }
    pub fn values_mut(&mut self) -> impl DoubleEndedIterator<Item=&mut T> { 
        self.0.values_mut() 
    }
}