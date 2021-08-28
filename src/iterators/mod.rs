use crate::id::IdLike;
use crate::structures::{ToOne, ToSet, VSet};

use std::collections::{BTreeMap, btree_set, btree_map};

use crate::moogcell::{InteriorBTreeMapRange, InteriorSetRange, InteriorVSet};

mod range_utils;

// == iterators ==
pub(crate) struct KeyValuesIterator<'a, Parent, K: IdLike, V: 'a> {
    iterator: InteriorBTreeMapRange<'a, Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: 'a> KeyValuesIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorBTreeMapRange<'a, Parent, K, V>) -> KeyValuesIterator<'a, Parent, K, V> {
        KeyValuesIterator {
            iterator,

            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>
    ) -> &mut btree_map::Range<'a, K, V> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute_arg(|xs| {
            range_utils::make_btreemap_range(open_parent(xs), fc, bc)
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>
    ) -> Option<(K, &'a V)> {
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let kv = iter.next();
        match kv {
            None => {self.done = true; None}
            Some((k, v)) => { self.front_cursor = Some(*k); Some((*k, v))}
        }
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>
    ) -> Option<(K, &'a V)> { 
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let kv = iter.next_back();
        match kv {
            None => {self.done = true; None}
            Some((k, v)) => { self.back_cursor = Some(*k); Some((*k, v))}
        }
    }
}

pub(crate) struct KeysIterator<'a, Parent, K: IdLike> {
    iterator: InteriorSetRange<'a, Parent, K>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike> KeysIterator<'a, Parent, K> {
    pub(crate) fn new(iterator: InteriorSetRange<'a, Parent, K>) -> KeysIterator<'a, Parent, K> {
        KeysIterator {
            iterator,

            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute<V: IdLike>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> &mut btree_set::Range<'a, K> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute_arg(|xs| {
            range_utils::make_toset_range(open_parent(xs), fc, bc)
        });
        iterator
    }

    pub(crate) fn next<V: IdLike>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> {
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next().map(|k| *k); 
        self.front_cursor = k; 
        if k == None { self.done = true; }
        k
    }

    pub(crate) fn next_back<V: IdLike>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<K> { 
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next_back().map(|k| *k); 
        self.back_cursor = k; 
        if k == None { self.done = true; }
        k
    }
}

pub(crate) struct SetIterator<'a, Parent, K: IdLike, V: IdLike> {
    cache: InteriorVSet<'a, Parent, K, V>,
    iterator: InteriorSetRange<'a, Parent, (K, V)>,

    key: K,
    front_cursor: Option<V>,
    back_cursor: Option<V>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: IdLike> SetIterator<'a, Parent, K, V> {
    pub(crate) fn new(
        cache: InteriorVSet<'a, Parent, K, V>,
        iterator: InteriorSetRange<'a, Parent, (K, V)>,
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
        find_vset: impl for<'b> FnOnce(&'b Parent, K) -> VSet<'b, K, V>
    ) -> &mut btree_set::Range<'a, (K, V)> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let key = self.key;

        let set: VSet<'a, K, V> = self.cache.get_or_compute_arg(|xs| { find_vset(xs, key) });
        let iterator = self.iterator.get_or_compute(|| { range_utils::make_vset_range(&set, fc, bc) });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> {
        if self.done { return None; }

        let iter = self.reconstitute(find_vset);
        match iter.next().map(|v| *v) {
            Some((_, v)) => {
                self.front_cursor = Some(v);
                Some(v)
            }
            None => {
                self.done = true;
                None
            }
        }
    }

    pub(crate) fn next_back(
        &mut self, 
        find_vset: impl FnOnce(&Parent, K) -> VSet<K, V>
    ) -> Option<V> { 
        if self.done { return None; }

        let iter = self.reconstitute(find_vset);
        match iter.next_back().map(|v| *v) {
            Some((_, v)) => {
                self.front_cursor = Some(v);
                Some(v)
            }
            None => {
                self.done = true;
                None
            }
        }
    }
}

pub(crate) struct FlatIterator<'a, Parent, K: IdLike, V: IdLike> {
    iterator: InteriorBTreeMapRange<'a, Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: IdLike> FlatIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorBTreeMapRange<'a, Parent, K, V>) -> FlatIterator<'a, Parent, K, V> {
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

        let iterator = self.iterator.get_or_compute_arg(|xs| {
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