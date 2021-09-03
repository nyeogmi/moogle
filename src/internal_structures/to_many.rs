use crate::id::IdLike;
use crate::methods::{EvictSet, ViewSet};

use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::btree_set;
use std::collections::btree_map;
use std::hash::{Hash, Hasher};
use std::ops::RangeBounds;

#[derive(Clone)]
pub(crate) struct ToMany<K, V> {
    keys: BTreeMap<K, Metadata>,
    elements: BTreeSet<(K, V)>,
}

#[derive(Clone)]
pub(crate) struct Metadata {
    count: usize,
}


impl<K: PartialEq<K>, V: PartialEq<V>> PartialEq for ToMany<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<K: Ord, V: Ord> PartialOrd<ToMany<K, V>> for ToMany<K, V> {
    fn partial_cmp(&self, other: &ToMany<K, V>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Hash, V: Hash> Hash for ToMany<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elements.hash(state);
    }
}

impl<K: Eq, V: Eq> Eq for ToMany<K, V> {

}

impl<K: Ord, V: Ord> Ord for ToMany<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.elements.cmp(&other.elements)
    }
}


impl<K: IdLike, V: IdLike> ToMany<K, V> {
    pub fn iter<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(K, V)> { 
        self.elements.iter().map(|(k, v)| (*k, *v))
    }

    pub fn keys<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=K> { 
        self.keys.iter().map(|(k, _)| *k) 
    }

    pub(crate) fn sets<'a>(&'a self) -> impl 'a+DoubleEndedIterator<Item=(K, VSet<'a, K, V>)> { 
        self.keys.iter().map(move |(k, _)| (*k, self.get(*k)))
    }
}

impl<'a, K: IdLike, V: IdLike> ToMany<K, V> {
    pub fn new() -> Self { ToMany { 
        keys: BTreeMap::new(),
        elements: BTreeSet::new(), 
    } }

    pub fn insert(&mut self, key: K, value: V, _on_evict: impl FnOnce(K, V)) -> Option<V> { 
        let is_new = self.elements.insert((key, value));

        // no benefit to calling the _on_evict callback because the opposed data structure it updates will imemdiately re-add this key
        // however, to caller, pretend we evicted
        if is_new { 
            match self.keys.entry(key) {
                btree_map::Entry::Occupied(mut v) => { v.get_mut().count += 1; }
                btree_map::Entry::Vacant(v) => { v.insert(Metadata { count: 1 }); }
            };
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
            self.elements.range((key, V::id_min_value())..=(key, V::id_max_value()))
        }
    }

    pub fn element_subrange(&self, k: impl RangeBounds<(K, V)>) -> btree_set::Range<'_, (K, V)> {
        self.elements.range(k)
    }

    pub fn key_subrange(&self, k: impl RangeBounds<K>) -> btree_map::Range<'_, K, Metadata> {
        self.keys.range(k)
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
            match self.keys.entry(key) {
                btree_map::Entry::Occupied(mut o) => {
                    let om = o.get_mut();
                    if om.count > 1 {
                        om.count -= 1;
                    } else {
                        o.remove_entry();
                    }
                }
                btree_map::Entry::Vacant(_) => {}
            }
            if self.key_range(key).next().is_none() { self.keys.remove(&key); }
            on_evict(key, value);
            return Some(value);
        }
        None
    }

    pub fn get(&'a self, key: K) -> VSet<'a, K, V> { VSet { key, map: self } }
    pub fn get_mut(&'a mut self, key: K) -> MSet<'a, K, V> { MSet { key, map: self } }
    pub fn contains_key(&self, key: K) -> bool { self.keys.contains_key(&key) }

    pub fn len(&self) -> usize { self.elements.len() }
    pub fn keys_len(&self) -> usize { self.keys.len() }
}

#[derive(Clone, Copy)]
pub(crate) struct VSet<'a, K: IdLike, V: IdLike> {
    key: K,
    map: &'a ToMany<K, V>,
}

pub(crate) struct MSet<'a, K: IdLike, V: IdLike> {
    key: K, 
    map: &'a mut ToMany<K, V>
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
        self.map.keys.get(&self.key).map_or(0, |f| f.count)
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

// == Standard traits ==
impl<A: IdLike, B: IdLike> IntoIterator for ToMany<A, B> {
    type Item = (A, B);

    type IntoIter = impl DoubleEndedIterator<Item=(A, B)>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}