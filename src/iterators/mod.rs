use crate::id::IdLike;
use crate::structures::{ToOne, ToSet};

use std::collections::{BTreeMap, btree_set, btree_map};

use crate::moogcell::{InteriorBTreeMapRange, InteriorSetRange};

mod range_utils;

// == iterators ==
pub(crate) struct BTreeMapIterator<'a, Parent, K: IdLike, V: 'a> {
    iterator: InteriorBTreeMapRange<'a, Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: 'a> BTreeMapIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorBTreeMapRange<'a, Parent, K, V>) -> BTreeMapIterator<'a, Parent, K, V> {
        BTreeMapIterator {
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

pub(crate) struct ToSetKeysIterator<'a, Parent, K: IdLike> {
    iterator: InteriorSetRange<'a, Parent, K>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike> ToSetKeysIterator<'a, Parent, K> {
    pub(crate) fn new(iterator: InteriorSetRange<'a, Parent, K>) -> ToSetKeysIterator<'a, Parent, K> {
        ToSetKeysIterator {
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
            range_utils::make_toset_key_range(open_parent(xs), fc, bc)
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

pub(crate) struct ToSetKeyValueIterator<'a, Parent, K: IdLike, V: IdLike> {
    iterator: InteriorSetRange<'a, Parent, (K, V)>,

    // unlike cursors, these aren't skipped
    front_element: Option<(K, V)>,
    back_element: Option<(K, V)>,

    front_cursor: Option<(K, V)>,
    back_cursor: Option<(K, V)>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: IdLike> ToSetKeyValueIterator<'a, Parent, K, V> {
    pub(crate) fn new(
        iterator: InteriorSetRange<'a, Parent, (K, V)>, 
        front_element: Option<(K, V)>,
        back_element: Option<(K, V)>,
    ) -> ToSetKeyValueIterator<'a, Parent, K, V> {
        ToSetKeyValueIterator {
            iterator,

            front_element, back_element,
            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> &mut btree_set::Range<'a, (K, V)> {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let fe = self.front_element;
        let be = self.back_element;

        let iterator = self.iterator.get_or_compute_arg(|xs| {
            range_utils::make_toset_key_value_range(
                open_parent(xs), fc, bc, fe, be
            )
        });
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<(K, V)> {
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next().map(|k| *k); 
        self.front_cursor = k; 
        if k == None { self.done = true; }
        k
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToSet<K, V>
    ) -> Option<(K, V)> { 
        if self.done { return None; }

        let iter = self.reconstitute(open_parent);
        let k = iter.next_back().map(|k| *k); 
        self.back_cursor = k; 
        if k == None { self.done = true; }
        k
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