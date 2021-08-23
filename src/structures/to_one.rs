use crate::keybound::Id;
use crate::methods::{EvictSetLike, ViewSetLike};

use std::collections::BTreeMap;

use auto_enums::auto_enum;

pub(crate) struct ToOne<K: Id, V: Id>(BTreeMap<K, V>);

impl<K: Id, V: Id> ToOne<K, V> {
    pub fn keys<'a>(&'a self) -> impl 'a+Iterator<Item=K> { self.0.keys().map(|k| *k) }
    pub fn iter<'a>(&'a self) -> impl 'a+Iterator<Item=(K, V)> { self.0.iter().map(|(k, v)| (*k, *v) ) }
    pub fn values<'a>(&'a self) -> impl 'a+Iterator<Item=V> { self.0.values().map(|v| *v ) }
}

impl<'a, K: Id, V: Id> ToOne<K, V> {
    pub fn new() -> Self { ToOne(BTreeMap::new()) }

    pub fn insert(&mut self, key: K, value: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        match self.0.get_mut(&key) {
            Some(x) => {
                if *x == value {
                    // skip on-evict callback, as we would just immediately put the value back into the opposed data structure
                    // but to caller, pretend we evicted
                    return Some(value)
                }

                let mut old = value;
                std::mem::swap(x, &mut old);
                on_evict(key, old);
                Some(old)
            }
            None => {
                self.0.insert(key, value);
                None
            }
        }
    }

    pub fn expunge(&mut self, key: K, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        let value = self.0.remove(&key);
        match value {
            Some(x) => { on_evict(key, x); Some(x) }
            None => None
        }
    }

    pub fn remove(&mut self, key: K, value: V, on_evict: impl FnOnce(K, V)) -> Option<V> {
        if self.0.get(&key) == Some(&value) {
            on_evict(key, value);
            self.0.remove(&key)
        } else {
            None
        }
    }

    pub fn get(&self, key: K) -> VOne<'a, K, V> { VOne(self.0.get(&key).map(|x| *x), ::std::marker::PhantomData) }
    pub fn contains_key(&self, key: K) -> bool { self.0.contains_key(&key) }
    pub fn len(&self) -> usize { self.0.len() }
}

pub(crate) struct VOne<'a, K: Id, V: Id>(Option<V>, ::std::marker::PhantomData<&'a *const K>);
pub(crate) struct MOne<'a, K: Id, V: Id>(K, &'a mut ToOne<K, V>);  

impl <'a, K: Id, V: Id> VOne<'a, K, V> {
    pub fn as_option(&self) -> Option<V> { self.0 }
}

impl<'a, K: Id, V: Id> EvictSetLike<'a, K, V> for MOne<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.insert(self.0, v, on_evict)
    }

    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.remove(self.0, v, on_evict)
    }
}

impl<'a, K: Id, V: Id> ViewSetLike<'a, V> for VOne<'a, K, V> {
    type Iter = impl 'a+Iterator<Item=V>;

    fn contains(&self, v: V) -> bool {
        match self.0 {
            None => false,
            Some(x) => x == v,
        }
    }

    fn len(&self) -> usize { 
        match self.0 {
            None => 0,
            Some(_) => 1,
        }
    }

    fn iter(&'a self) -> Self::Iter { 
        self.0.iter().map(|v| *v)
    }
}

impl<'a, K: Id, V: Id> ViewSetLike<'a, V> for MOne<'a, K, V> {
    type Iter = impl 'a+Iterator<Item=V>;

    fn contains(&self, v: V) -> bool { 
        match self.1.0.get(&self.0) {
            None => false,
            Some(x) => *x == v,
        }
    }

    fn len(&self) -> usize { 
        match self.1.0.get(&self.0) {
            None => 0,
            Some(_) => 1,
        }
    }

    #[auto_enum(Iterator)]
    fn iter(&'a self) -> Self::Iter { 
        match self.1.0.get(&self.0) {
            None => std::iter::empty::<V>(),
            Some(x) => std::iter::once::<V>(*x),
        }
    }
}