use crate::Id;

use std::collections::BTreeMap;

mod debug_impl;

pub struct RawPom<T> { 
    next_id: u64,
    pub(crate) members: BTreeMap<Id<T>, T>
}

impl<T> RawPom<T> {
    pub fn new() -> Self {
        RawPom {
            next_id: 1,
            members: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> Id<T> {
        let id = Id::new(self.next_id);
        self.next_id += 1;
        self.members.insert(id, t);
        id
    }
    pub fn remove(&mut self, k: Id<T>) -> Option<T> { self.members.remove(&k) }

    pub fn get(&self, k: Id<T>) -> Option<&T> { self.members.get(&k) }
    pub fn get_mut(&mut self, k: Id<T>) -> Option<&mut T> { self.members.get_mut(&k) }
    pub fn contains_key(&self, k: Id<T>) -> bool { self.members.contains_key(&k) }
    pub fn len(&self) -> usize { self.members.len() }

    pub fn iter<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(Id<T>, &'a T)> { 
        self.members.iter().map(|(k, v)| (*k, v)) 
    }
    pub fn iter_mut<'a>(&'a mut self) -> impl 'a+DoubleEndedIterator<Item=(Id<T>, &'a mut T)> { 
        self.members.iter_mut().map(|(k, v)| (*k, v)) 
    }
    pub fn keys<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=Id<T>> { 
        self.members.keys().cloned() 
    }
    pub fn values<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=&'a T> { 
        self.members.values() 
    }
    pub fn values_mut<'a>(&'a mut self) -> impl 'a+DoubleEndedIterator<Item=&'a mut T> { 
        self.members.values_mut() 
    }
}