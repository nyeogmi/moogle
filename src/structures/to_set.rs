use crate::keybound::Id;
use crate::methods::{EvictSet, ViewSet};

use std::collections::{BTreeSet, BTreeMap};
use std::collections::btree_map::Range;
use std::ops::RangeBounds;

pub(crate) struct ToSet<K, V> {
    elements: BTreeMap<K, BTreeSet<V>>,
    total_len: usize,
}

impl<K: Id, V: Id> ToSet<K, V> {
    pub fn iter<'a>(&'a self) -> impl 'a+Iterator<Item=(K, V)> { 
        self.elements.iter().flat_map(|(k, vs)|
            vs.iter().map(move |v| (*k, *v))
        )
    }
    pub fn keys<'a>(&'a self) -> impl 'a+Iterator<Item=K> { self.elements.keys().map(|k| *k) }
    pub fn sets<'a>(&'a self) -> impl 'a+Iterator<Item=(K, &BTreeSet<V>)> { self.elements.iter().map(|(k, v)| (*k, v) ) }
}

// TODO: Track _total_ len (as in, number of pairs)
impl<'a, K: Id, V: Id> ToSet<K, V> {
    pub fn new() -> Self { ToSet { elements: BTreeMap::new(), total_len: 0 } }

    pub fn insert(&mut self, key: K, value: V, _on_evict: impl FnOnce(K, V)) -> Option<V> { 
        let is_new = self.elements.entry(key).or_insert_with(|| BTreeSet::new()).insert(value);

        // no benefit to calling the _on_evict callback because the opposed data structure it updates will imemdiately re-add this key
        // however, to caller, pretend we evicted
        if is_new { 
            self.total_len += 1; 
            None 
        } else { 
            Some(value) 
        }
    }

    pub fn range(&self, r: impl RangeBounds<K>) -> Range<'_, K, BTreeSet<V>> {
        self.elements.range(r)
    }

    pub fn expunge(&mut self, key: K, mut on_evict: impl FnMut(K, V)) -> BTreeSet<V> {
        match self.elements.remove(&key) {
            Some(xs) => {
                for x in xs.iter() { on_evict(key, *x); }
                self.total_len -= xs.len();
                xs
            }
            None => {
                BTreeSet::new()
            }
        }
    }

    pub fn remove(&mut self, key: K, value: V, on_evict: impl FnOnce(K, V)) -> Option<V> {
        let (result, len) = match self.elements.get_mut(&key) {
            Some(xs) => { 
                let result = xs.take(&value); 
                if result.is_some() { 
                    on_evict(key, value); 
                    self.total_len -= 1;
                }
                (result, xs.len())
            }
            None => { return None }
        };
        if len == 0 { self.elements.remove(&key); };
        result
    }

    pub fn get(&'a self, key: K) -> VSet<'a, K, V> { 
        VSet(self.elements.get(&key), ::std::marker::PhantomData) 
    }
    pub fn get_mut(&'a mut self, key: K) -> MSet<'a, K, V> { MSet(key, self) }
    pub fn contains_key(&self, key: K) -> bool { self.elements.contains_key(&key) }

    pub fn len(&self) -> usize { self.total_len }
    pub fn keys_len(&self) -> usize { self.elements.len() }
}

#[derive(Clone, Copy)]
pub(crate) struct VSet<'a, K: Id, V: Id>(pub(crate) Option<&'a BTreeSet<V>>, ::std::marker::PhantomData<*const K>);
pub(crate) struct MSet<'a, K: Id, V: Id>(K, &'a mut ToSet<K, V>);  

impl<'a, K: Id, V: Id> MSet<'a, K, V> {
    pub fn key(&self) -> K { self.0 }
}

impl<'a, K: Id, V: Id> EvictSet<'a, K, V> for MSet<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.insert(self.0, v, on_evict)
    }

    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.remove(self.0, v, on_evict)
    }
}


impl<'a, K: Id, V: Id> ViewSet<'a, V> for VSet<'a, K, V> {
    type Iter = impl 'a+Iterator<Item=V>;

    fn contains(&self, v: V) -> bool {
        match self.0 {
            None => false,
            Some(x) => x.contains(&v),
        }
    }

    fn len(&self) -> usize { 
        match self.0 {
            None => 0,
            Some(x) => x.len(),
        }
    }

    fn iter(&self) -> Self::Iter { 
        self.0.into_iter().flat_map(|vs| vs.iter()).map(|v| *v)
    }
}

impl<'a, K: Id, V: Id> ViewSet<'a, V> for MSet<'a, K, V> {
    type Iter = impl Iterator<Item=V>;

    fn contains(&self, v: V) -> bool { 
        match self.1.elements.get(&self.0) {
            None => false,
            Some(x) => x.contains(&v),
        }
    }

    fn len(&self) -> usize { 
        match self.1.elements.get(&self.0) {
            None => 0,
            Some(x) => x.len(),
        }
    }

    fn iter(&'a self) -> Self::Iter { 
        self.1.elements.get(&self.0).into_iter().flat_map(|vs| vs.iter()).map(|v| *v)
    }
}