use crate::keybound::Id;
use crate::structures::{ToOne, ToSet, VSet};

use std::collections::{BTreeSet, btree_set, btree_map};

use super::range_utils;
use super::moogcell::{InteriorSetRange, InteriorTreeRange, InteriorVSet};

// == iterators ==
pub(crate) struct KeysIterator<'a, Parent, K: Id, V: Id> {
    iterator: InteriorTreeRange<'a, Parent, K, BTreeSet<V>>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: Id, V: Id> KeysIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorTreeRange<'a, Parent, K, BTreeSet<V>>) -> KeysIterator<'a, Parent, K, V> {
        KeysIterator {
            iterator,

            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> &mut btree_map::Range<'a, K, BTreeSet<V>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(|xs| {
            range_utils::make_toset_range(open_parent(xs), fc, bc)
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> {
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next().map(|(k, _)| *k); 
        self.front_cursor = k; 
        if k == None { self.done = true; }
        k
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> { 
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next_back().map(|(k, _)| *k); 
        self.back_cursor = k; 
        if k == None { self.done = true; }
        k
    }
}

pub(crate) struct SetIterator<'a, Parent, K: Id, V: Id> {
    cache: InteriorVSet<'a, Parent, K, V>,
    iterator: InteriorSetRange<'a, Parent, V>,

    key: K,
    front_cursor: Option<V>,
    back_cursor: Option<V>,
    done: bool,
}

impl<'a, Parent, K: Id, V: Id> SetIterator<'a, Parent, K, V> {
    pub(crate) fn new(
        cache: InteriorVSet<'a, Parent, K, V>,
        iterator: InteriorSetRange<'a, Parent, V>,
        key: K,
    ) -> SetIterator<'a, Parent, K, V> {
        SetIterator {
            cache,
            iterator,

            key,
            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    pub(crate) fn reconstitute(
        &mut self,
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<&mut btree_set::Range<'a, V>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let key = self.key;

        let set = self.cache.get_or_compute(|xs| {
            find_vset(xs, key)
        });
        let bt = match set.0 {
            None => return None,
            Some(b) => b,
        };
        let iterator = self.iterator.get_or_compute(|| {
            range_utils::make_btreeset_range(bt, fc, bc)
        });
        Some(iterator)
    }

    pub(crate) fn next(
        &mut self, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> {
        if self.done { return None; }

        let iter = self.reconstitute(find_vset);
        let v = iter?.next().map(|v| *v); 
        self.front_cursor = v; 
        if v == None { self.done = true; }
        v
    }

    pub(crate) fn next_back(
        &mut self, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> { 
        if self.done { return None; }

        let iter = self.reconstitute(find_vset);
        let v = iter?.next_back().map(|v| *v); 
        self.back_cursor = v; 
        if v == None { self.done = true; }
        v
    }
}

pub(crate) struct FlatIterator<'a, Parent, K: Id, V: Id> {
    iterator: InteriorTreeRange<'a, Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: Id, V: Id> FlatIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorTreeRange<'a, Parent, K, V>) -> FlatIterator<'a, Parent, K, V> {
        FlatIterator {
            iterator,
            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(|xs| {
            range_utils::make_btreemap_range(&open_parent(xs).0, fc, bc)
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> Option<(K, V)> {
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        match iter.next().map(|(k, v)| (*k, *v)) {
            Some((k, v)) => {
                self.front_cursor = Some(k);
                Some((k, v))
            },
            None => { 
                self.done = true;
                None
            }
        }
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> Option<(K, V)> { 
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        match iter.next_back().map(|(k, v)| (*k, *v)) {
            Some((k, v)) => {
                self.back_cursor = Some(k);
                Some((k, v))
            },
            None => { 
                self.done = true;
                None
            }
        }
    }
}