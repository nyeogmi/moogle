use crate::keybound::Id;
use crate::structures::{ToOne, ToSet, VSet};

use std::collections::{BTreeSet, btree_set, btree_map};

use super::range_utils;
use super::moogcell::{MoogCell, InteriorSetRange, InteriorTreeRange, InteriorVSet};

// == iterators ==
pub(crate) struct KeysIterator<Parent, K: Id, V: Id> {
    iterator: InteriorTreeRange<Parent, K, BTreeSet<V>>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
}

impl<Parent, K: Id, V: Id> KeysIterator<Parent, K, V> {
    pub(crate) fn new(iterator: InteriorTreeRange<Parent, K, BTreeSet<V>>) -> KeysIterator<Parent, K, V> {
        KeysIterator {
            iterator,
            front_cursor: None,
            back_cursor: None,
        }
    }

    fn reconstitute<'a>(
        &mut self, 
        parent: &'a MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> &mut btree_map::Range<'a, K, BTreeSet<V>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(parent, |xs| {
            range_utils::make_toset_range(open_parent(xs), fc, bc)
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        parent: &MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> {
        let iter = self.reconstitute(parent, open_parent);
        let k = iter.next().map(|(k, _)| *k); 
        self.front_cursor = k; 
        k
    }

    pub(crate) fn next_back(
        &mut self, 
        parent: &MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> { 
        let iter = self.reconstitute(parent, open_parent);
        let k = iter.next_back().map(|(k, _)| *k); 
        self.back_cursor = k; 
        k
    }
}

pub(crate) struct SetIterator<Parent, K: Id, V: Id> {
    cache: InteriorVSet<Parent, K, V>,
    iterator: InteriorSetRange<Parent, V>,

    key: K,
    front_cursor: Option<V>,
    back_cursor: Option<V>,
}

impl<Parent, K: Id, V: Id> SetIterator<Parent, K, V> {
    pub(crate) fn new(
        cache: InteriorVSet<Parent, K, V>,
        iterator: InteriorSetRange<Parent, V>,
        key: K,
    ) -> SetIterator<Parent, K, V> {
        SetIterator {
            cache,
            iterator,

            key,
            front_cursor: None,
            back_cursor: None,
        }
    }

    pub(crate) fn reconstitute<'a>(
        &mut self,
        parent: &'a MoogCell<Parent>, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<&mut btree_set::Range<'a, V>> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let key = self.key;

        let set = self.cache.get_or_compute(parent, |xs| {
            find_vset(xs, key)
        });
        let bt = match set.0 {
            None => return None,
            Some(b) => b,
        };
        let iterator = self.iterator.get_or_compute(parent, || {
            range_utils::make_btreeset_range(bt, fc, bc)
        });
        Some(iterator)
    }

    pub(crate) fn next<'a>(
        &mut self, 
        parent: &'a MoogCell<Parent>, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> {
        let iter = self.reconstitute(parent, find_vset);
        let v = iter?.next().map(|v| *v); 
        self.front_cursor = v; 
        v
    }

    pub(crate) fn next_back<'a>(
        &mut self, 
        parent: &'a MoogCell<Parent>, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> { 
        let iter = self.reconstitute(parent, find_vset);
        let v = iter?.next_back().map(|v| *v); 
        self.back_cursor = v; 
        v
    }
}

pub(crate) struct FlatIterator<Parent, K: Id, V: Id> {
    iterator: InteriorTreeRange<Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
}

impl<Parent, K: Id, V: Id> FlatIterator<Parent, K, V> {
    pub(crate) fn new(iterator: InteriorTreeRange<Parent, K, V>) -> FlatIterator<Parent, K, V> {
        FlatIterator {
            iterator,
            front_cursor: None,
            back_cursor: None,
        }
    }

    fn reconstitute<'a>(
        &mut self, 
        parent: &'a MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(parent, |xs| {
            range_utils::make_btreemap_range(&open_parent(xs).0, fc, bc)
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        parent: &MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> Option<(K, V)> {
        let iter = self.reconstitute(parent, open_parent);
        match iter.next().map(|(k, v)| (*k, *v)) {
            Some((k, v)) => {
                self.front_cursor = Some(k);
                Some((k, v))
            },
            None => { 
                self.front_cursor = None; 
                None
            }
        }
    }

    pub(crate) fn next_back(
        &mut self, 
        parent: &MoogCell<Parent>, 
        open_parent: impl FnOnce(&Parent) -> &ToOne<K, V>
    ) -> Option<(K, V)> { 
        let iter = self.reconstitute(parent, open_parent);
        match iter.next_back().map(|(k, v)| (*k, *v)) {
            Some((k, v)) => {
                self.back_cursor = Some(k);
                Some((k, v))
            },
            None => { 
                self.back_cursor = None; 
                None
            }
        }
    }
}