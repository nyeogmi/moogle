use std::collections::{BTreeSet, BTreeMap};

use super::super::interfaces::{EvictSetLike, ViewSetLike};

pub struct ToOne<K, V>(BTreeMap<K, V>);

impl<'a, K: Ord, V: Ord> ToOne<K, V> {
    pub fn insert(&mut self, key: K, value: V, on_evict: impl FnOnce(&K, &V)) -> Option<V> { 
        match self.0.get_mut(&key) {
            Some(x) => {
                on_evict(&key, x);
                let mut old = value;
                std::mem::swap(x, &mut old);
                Some(old)
            }
            None => {
                self.0.insert(key, value);
                None
            }
        }
    }

    pub fn expunge(&mut self, key: &K, on_evict: impl FnOnce(&K, &V)) -> Option<V> { 
        let value = self.0.remove(key);
        match value {
            Some(x) => { on_evict(key, &x); Some(x) }
            None => None
        }
    }

    pub fn remove(&mut self, key: &K, value: &V, on_evict: impl FnOnce(&K, &V)) -> Option<V> {
        if self.0.get(key) == Some(value) {
            on_evict(key, value);
            self.0.remove(key)
        } else {
            None
        }
    }

    pub fn get(&'a self, key: &K) -> VOne<'a, K, V> { VOne(self.0.get(key), ::std::marker::PhantomData) }
    pub fn get_mut(&'a mut self, key: K) -> MOne<'a, K, V> { MOne(key, self) }
    pub fn contains_key(&self, key: &K) -> bool { self.0.contains_key(key) }
    pub fn len(&self) -> usize { self.0.len() }
}

pub struct VOne<'a, K, V>(Option<&'a V>, ::std::marker::PhantomData<*const K>);
pub struct MOne<'a, K, V>(K, &'a mut ToOne<K, V>);  

impl <'a, K, V> VOne<'a, K, V> {
    pub fn as_option(&self) -> Option<&'a V> { self.0 }
}

impl<'a, K: Ord, V> MOne<'a, K, V> {
    pub fn key(&self) -> &K { &self.0 }
    // pub fn as_option(&'a mut self) -> Option<&'a mut V> { self.1.0.entry(self.0.clone()).get_mut }
}

impl<'a, K: Ord+Clone, V: Ord> EvictSetLike<K, V> for MOne<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(&K, &V)) -> bool { 
        // TODO: Use self.1.insert()? But we don't want to do an extra clone, probably.
        match self.1.0.get_mut(&self.0) {
            Some(x) => {
                let existed = *x == v;
                if existed { return true; }
                on_evict(&self.0, x);
                *x = v;
            }
            None => {
                self.1.0.insert(self.0.clone(), v);  // TODO: Shouldn't need to clone, as this only ever happens once
            }
        };
        false
    }

    fn remove(&mut self, v: &V, on_evict: impl FnOnce(&K, &V)) -> Option<V> { 
        self.1.remove(&self.0, v, on_evict)
    }
}


impl<'a, K: Ord, V: Ord> ViewSetLike<V> for VOne<'a, K, V> {
    fn contains(&self, v: &V) -> bool {
        match self.0 {
            None => false,
            Some(x) => x == v,
        }
    }

    fn len(&self) -> usize { 
        match self.0 {
            None => 0,
            Some(x) => 1,
        }
    }
}

impl<'a, K: Ord, V: Ord> ViewSetLike<V> for MOne<'a, K, V> {
    fn contains(&self, v: &V) -> bool { 
        match self.1.0.get(&self.0) {
            None => false,
            Some(x) => x == v,
        }
    }

    fn len(&self) -> usize { 
        match self.1.0.get(&self.0) {
            None => 0,
            Some(x) => 1,
        }
    }
}