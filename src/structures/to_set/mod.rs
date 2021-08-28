// TODO: Use unwrap_unchecked for min_v/max_v, or possibly MaybeUninit
use crate::id::IdLike;
use crate::methods::{EvictSet, ViewSet};

use std::collections::BTreeSet;
use std::collections::btree_set;
use std::ops::RangeBounds;

pub(crate) struct ToSet<K, V> {
    min_max_v: Option<(V, V)>, 
    keys: BTreeSet<K>,
    elements: BTreeSet<(K, V)>,
}


impl<K: IdLike, V: IdLike> ToSet<K, V> {
    pub fn iter<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(K, V)> { 
        self.elements.iter().map(|(k, v)| (*k, *v))
    }

    pub fn keys<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=K> { 
        self.keys.iter().map(|k| *k) 
    }

    pub(crate) fn sets<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(K, VSet<'a, K, V>)> { 
        self.keys.iter().map(move |k| (*k, self.get(*k)))
    }
}

// TODO: Track _total_ len (as in, number of pairs)
impl<'a, K: IdLike, V: IdLike> ToSet<K, V> {
    pub fn new() -> Self { ToSet { 
        keys: BTreeSet::new(),
        elements: BTreeSet::new(), 
        min_max_v: None,
    } }

    fn update_min_max(&mut self, value: V) {
        self.min_max_v = Some(match self.min_max_v {
            None => (value, value),
            Some((mn, mx)) => (mn.min(value), mx.max(value)),
        })
    }

    pub fn insert(&mut self, key: K, value: V, _on_evict: impl FnOnce(K, V)) -> Option<V> { 
        let is_new = self.elements.insert((key, value));

        // no benefit to calling the _on_evict callback because the opposed data structure it updates will imemdiately re-add this key
        // however, to caller, pretend we evicted
        if is_new { 
            self.keys.insert(key);
            self.update_min_max(value);
            None 
        } else { 
            Some(value) 
        }
    }

    fn key_range(&self, key: K) -> btree_set::Range<'_, (K, V)> {
        if self.elements.is_empty() {
            // doesn't matter
            self.elements.range(..)
        } else { 

            // TODO: Use unwrap_unchecked
            let (min_v, max_v) = self.min_max_v.unwrap();
            self.elements.range((key, min_v)..=(key, max_v))
        }
    }

    pub fn element_subrange(&self, k: impl RangeBounds<(K, V)>) -> btree_set::Range<'_, (K, V)> {
        self.elements.range(k)
    }

    pub fn key_subrange(&self, k: impl RangeBounds<K>) -> btree_set::Range<'_, K> {
        self.keys.range(k)
    }

    fn key_value_subrange(&self, key: K, v0: Option<V>, v1: Option<V>) -> btree_set::Range<'_, (K, V)> {
        if self.elements.is_empty() {
            // doesn't matter
            self.elements.range(..)
        } else {
            // TODO: Use unwrap_unchcked
            let (min_v, max_v) = self.min_max_v.unwrap();
            let v0_real = match v0 {
                // more restrictive
                Some(x) => x,
                None => min_v,
            };
            let v1_real = match v1 {
                Some(x) => x,
                None => max_v,
            };
            self.elements.range((key, v0_real)..=(key, v1_real))
        }

    }

    pub fn expunge(&mut self, key: K, mut on_evict: impl FnMut(K, V)) -> BTreeSet<V> {
        let mut values = BTreeSet::new();
        for (_, v) in self.key_range(key) {
            values.insert(*v);
            on_evict(key, *v);
        }
        // TODO: Drain a range?
        for v in values.iter() {
            self.elements.remove(&(key, *v));
        }
        values
    }

    pub fn remove(&mut self, key: K, value: V, on_evict: impl FnOnce(K, V)) -> Option<V> {
        if self.elements.remove(&(key, value)) {
            if self.key_range(key).next().is_none() { self.keys.remove(&key); }
            on_evict(key, value);
            return Some(value);
        }
        None
    }

    pub fn get(&'a self, key: K) -> VSet<'a, K, V> { VSet { key, map: self } }
    pub fn get_mut(&'a mut self, key: K) -> MSet<'a, K, V> { MSet { key, map: self } }
    pub fn contains_key(&self, key: K) -> bool { self.keys.contains(&key) }

    pub fn len(&self) -> usize { self.elements.len() }
    pub fn keys_len(&self) -> usize { self.keys.len() }
}

#[derive(Clone, Copy)]
pub(crate) struct VSet<'a, K: IdLike, V: IdLike> {
    key: K,
    map: &'a ToSet<K, V>,
}

impl<'a, K: IdLike, V: IdLike> VSet<'a, K, V> {
    pub(crate) unsafe fn unsafe_transmute_lifetime<'b>(&self) -> VSet<'b, K, V> {
        VSet { key: self.key, map: std::mem::transmute(self.map) }
    }
}

pub(crate) struct MSet<'a, K: IdLike, V: IdLike> {
    key: K, 
    map: &'a mut ToSet<K, V>
}

impl<'a, K: IdLike, V: IdLike> MSet<'a, K, V> {
    pub fn key(&self) -> K { self.key }
}

impl<'a, K: IdLike, V: IdLike> EvictSet<'a, K, V> for MSet<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.map.insert(self.key, v, on_evict)
    }

    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.map.remove(self.key, v, on_evict)
    }
}


impl<'a, K: IdLike, V: IdLike> ViewSet<'a, V> for VSet<'a, K, V> {
    type Iter = impl 'a+DoubleEndedIterator<Item=V>;

    fn contains(&self, v: V) -> bool {
        self.map.elements.contains(&(self.key, v))
    }

    fn len(&self) -> usize { 
        // TODO: This is a terrible implementation and should be done with a separate struct under keys
        self.map.key_range(self.key).count()
    }

    fn iter(&self) -> Self::Iter { 
        self.map.key_range(self.key).map(|(_, v)| *v)
    }
}

impl<'a, K: IdLike, V: IdLike> ViewSet<'a, V> for MSet<'a, K, V> {
    type Iter = impl DoubleEndedIterator<Item=V>;

    fn contains(&self, v: V) -> bool { 
        self.map.elements.contains(&(self.key, v))
    }

    fn len(&self) -> usize { 
        // TODO: This is a terrible implementation and should be done with a separate struct under keys
        self.map.key_range(self.key).count()
    }

    fn iter(&'a self) -> Self::Iter { 
        self.map.key_range(self.key).map(|(_, v)| *v)
    }
}

impl<'a, K: IdLike, V: IdLike+std::fmt::Debug> std::fmt::Debug for VSet<'a, K, V> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        fmt.debug_set().entries(self.iter()).finish()
    }
}

impl<'a, K: IdLike, V: IdLike> VSet<'a, K, V> {
    pub(crate) fn range(&self, v0: Option<V>, v1: Option<V>) -> btree_set::Range<'a, (K, V)> {
        self.map.key_value_subrange(self.key, v0, v1)
    }
}