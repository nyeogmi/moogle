use crate::keybound::Id;
use crate::methods::{EvictSetLike, ViewSetLike};

use std::collections::{BTreeSet, BTreeMap};

use auto_enums::auto_enum;

pub(crate) struct ToSet<K, V>(BTreeMap<K, BTreeSet<V>>);

impl<K: Id, V: Id> ToSet<K, V> {
    pub fn items<'a>(&'a self) -> impl 'a+Iterator<Item=(K, V)> { 
        self.0.iter().flat_map(|(k, vs)|
            vs.iter().map(move |v| (*k, *v))
        )
    }
    pub fn keys<'a>(&'a self) -> impl 'a+Iterator<Item=K> { self.0.keys().map(|k| *k) }
    pub fn sets<'a>(&'a self) -> impl 'a+Iterator<Item=(K, &BTreeSet<V>)> { self.0.iter().map(|(k, v)| (*k, v) ) }
}

// TODO: Track _total_ len (as in, number of pairs)
impl<'a, K: Id, V: Id> ToSet<K, V> {
    pub fn new() -> Self { ToSet(BTreeMap::new()) }

    pub fn insert(&mut self, key: K, value: V, _on_evict: impl FnOnce(K, V)) -> Option<V> { 
        let existing = self.0.entry(key).or_insert_with(|| BTreeSet::new()).insert(value);

        // no benefit to calling the _on_evict callback because the opposed data structure it updates will imemdiately re-add this key
        // however, to caller, pretend we evicted
        if existing { Some(value) } else { None }
    }

    pub fn expunge(&mut self, key: K, mut on_evict: impl FnMut(K, V)) -> BTreeSet<V> {
        match self.0.remove(&key) {
            Some(xs) => {
                for x in xs.iter() { on_evict(key, *x); }
                xs
            }
            None => {
                BTreeSet::new()
            }
        }
    }

    pub fn remove(&mut self, key: K, value: V, on_evict: impl FnOnce(K, V)) -> Option<V> {
        let (result, len) = match self.0.get_mut(&key) {
            Some(xs) => { 
                let result = xs.take(&value); 
                if result.is_some() { on_evict(key, value); }
                (result, xs.len())
            }
            None => { return None }
        };
        if len == 0 { self.0.remove(&key); };
        result
    }

    pub fn get(&'a self, key: K) -> VSet<'a, K, V> { VSet(self.0.get(&key), ::std::marker::PhantomData) }
    pub fn get_mut(&'a mut self, key: K) -> MSet<'a, K, V> { MSet(key, self) }
    pub fn contains_key(&self, key: K) -> bool { self.0.contains_key(&key) }
    pub fn len(&self) -> usize { self.0.len() }
}

pub(crate) struct VSet<'a, K: Id, V: Id>(Option<&'a BTreeSet<V>>, ::std::marker::PhantomData<*const K>);
pub(crate) struct MSet<'a, K: Id, V: Id>(K, &'a mut ToSet<K, V>);  

impl<'a, K: Id, V: Id> MSet<'a, K, V> {
    pub fn key(&self) -> K { self.0 }
}

impl<'a, K: Id, V: Id> EvictSetLike<'a, K, V> for MSet<'a, K, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.insert(self.0, v, on_evict)
    }

    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V> { 
        self.1.remove(self.0, v, on_evict)
    }
}


impl<'a, K: Id, V: Id> ViewSetLike<'a, V> for VSet<'a, K, V> {
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

    #[auto_enum(Iterator)]
    fn iter(&self) -> Self::Iter { 
        match self.0 {
            None => std::iter::empty::<V>(),
            Some(x) => x.iter().map(|v| *v)
        }
    }
}

impl<'a, K: Id, V: Id> ViewSetLike<'a, V> for MSet<'a, K, V> {
    type Iter = impl Iterator<Item=V>;

    fn contains(&self, v: V) -> bool { 
        match self.1.0.get(&self.0) {
            None => false,
            Some(x) => x.contains(&v),
        }
    }

    fn len(&self) -> usize { 
        match self.1.0.get(&self.0) {
            None => 0,
            Some(x) => x.len(),
        }
    }

    #[auto_enum(Iterator)]
    fn iter(&'a self) -> Self::Iter { 
        match self.1.0.get(&self.0) {
            None => std::iter::empty::<V>(),
            Some(x) => x.iter().map(|v| *v)
        }
    }
}