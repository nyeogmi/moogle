use std::collections::{BTreeSet, BTreeMap};

use super::super::interfaces::{EvictSetLike, ViewSetLike};

pub struct ToSet<K, V>(BTreeMap<K, BTreeSet<V>>);

// TODO: Track _total_ len (as in, number of pairs)
impl<'a, K: Ord, V: Ord> ToSet<K, V> {
    pub fn insert(&mut self, key: K, value: V, _on_evict: impl FnOnce(&K, &V)) { 
        self.0.entry(key).or_insert_with(|| BTreeSet::new()).insert(value);
    }

    pub fn expunge(&mut self, key: &K, mut on_evict: impl FnMut(&K, &V)) -> BTreeSet<V> {
        match self.0.remove(key) {
            Some(xs) => {
                for x in xs.iter() { on_evict(key, x); }
                xs
            }
            None => {
                BTreeSet::new()
            }
        }
    }

    pub fn remove(&mut self, key: &K, value: &V, on_evict: impl FnOnce(&K, &V)) -> Option<V> {
        let (result, len) = match self.0.get_mut(&key) {
            Some(xs) => { 
                let result = xs.take(value); 
                if result.is_some() { on_evict(key, value); }
                (result, xs.len())
            }
            None => { return None }
        };
        if len == 0 { self.0.remove(&key); };
        result
    }

    pub fn get(&'a self, key: &K) -> VSet<'a, K, V> { VSet(self.0.get(key), ::std::marker::PhantomData) }
    pub fn get_mut(&'a mut self, key: K) -> MSet<'a, K, V> { MSet(key, self) }
    pub fn contains_key(&self, key: &K) -> bool { self.0.contains_key(key) }
    pub fn len(&self) -> usize { self.0.len() }
}

pub struct VSet<'a, K, V>(Option<&'a BTreeSet<V>>, ::std::marker::PhantomData<*const K>);

// TODO: Use the entry interface only. The problem is that entries can't be used more than once
pub struct MSet<'a, K, V>(K, &'a mut ToSet<K, V>);  

impl<'a, K, V> MSet<'a, K, V> {
    pub fn key(&self) -> &K { &self.0 }
}

impl<'a, K: Ord+Clone, V: Ord> EvictSetLike<K, V> for MSet<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(&K, &V)) -> bool { 
        // TODO: Use self.1.insert()? But we don't want to do an extra clone, probably.
        match self.1.0.get_mut(&self.0) {
            Some(xs) => {
                xs.insert(v)
            }
            None => {
                let mut s = BTreeSet::new();
                s.insert(v);
                self.1.0.insert(self.0.clone(), s);  // TODO: Shouldn't need to clone, as this only ever happens once
                false
            }
        }
    }

    fn remove(&mut self, v: &V, on_evict: impl FnOnce(&K, &V)) -> Option<V> { 
        self.1.remove(&self.0, v, on_evict)
    }
}


impl<'a, K: Ord, V: Ord> ViewSetLike<V> for VSet<'a, K, V> {
    fn contains(&self, v: &V) -> bool {
        match self.0 {
            None => false,
            Some(x) => x.contains(v),
        }
    }

    fn len(&self) -> usize { 
        match self.0 {
            None => 0,
            Some(x) => x.len(),
        }
    }
}

impl<'a, K: Ord, V: Ord> ViewSetLike<V> for MSet<'a, K, V> {
    fn contains(&self, v: &V) -> bool { 
        match self.1.0.get(&self.0) {
            None => false,
            Some(x) => x.contains(v),
        }
    }

    fn len(&self) -> usize { 
        match self.1.0.get(&self.0) {
            None => 0,
            Some(x) => x.len(),
        }
    }
}