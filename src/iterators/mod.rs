use crate::id::IdLike;
use crate::internal_structures::{ToMany, ToManyMetadata};

use std::collections::{BTreeMap, btree_set, btree_map};

use crate::moogcell::{InteriorMapRange, InteriorSetRange};

mod range_utils;

// == iterators ==
pub(crate) struct BTreeMapIterator<'a, Parent, K: IdLike, V: 'a> {
    iterator: InteriorMapRange<'a, Parent, K, V>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: Copy> BTreeMapIterator<'a, Parent, K, V> {
    pub(crate) fn new(iterator: InteriorMapRange<'a, Parent, K, V>) -> BTreeMapIterator<'a, Parent, K, V> {
        BTreeMapIterator {
            iterator,

            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute<T>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>,
        body: impl FnOnce(&mut btree_map::Range<'_, K, V>) -> T
    ) -> T {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        let iterator = self.iterator.get_or_compute(
            |xs| { range_utils::make_map_range(open_parent(xs), fc, bc) },
            |range| { body(range) },
        );
        iterator
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>
    ) -> Option<(K, V)> {
        if self.done { return None; }

        let kv = self.reconstitute(open_parent, |iter| { iter.next().map(|(k, v)| (*k, *v)) });
        match kv {
            None => {self.done = true; None}
            Some((k, v)) => { self.front_cursor = Some(k); Some((k, v))}
        }
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &BTreeMap<K, V>
    ) -> Option<(K, V)> { 
        if self.done { return None; }

        let kv = self.reconstitute(open_parent, |iter| { iter.next_back().map(|(k, v)| (*k, *v)) });
        match kv {
            None => {self.done = true; None}
            Some((k, v)) => { self.back_cursor = Some(k); Some((k, v))}
        }
    }
}

pub(crate) struct ToManyKeysIterator<'a, Parent, K: IdLike> {
    iterator: InteriorMapRange<'a, Parent, K, ToManyMetadata>,

    front_cursor: Option<K>,
    back_cursor: Option<K>,
    done: bool,
}

impl<'a, Parent, K: IdLike> ToManyKeysIterator<'a, Parent, K> {
    pub(crate) fn new(iterator: InteriorMapRange<'a, Parent, K, ToManyMetadata>) -> ToManyKeysIterator<'a, Parent, K> {
        ToManyKeysIterator {
            iterator,

            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute<V: IdLike, T>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>,
        body: impl FnOnce(&mut btree_map::Range<'_, K, ToManyMetadata>) -> T
    ) -> T {
        let fc = self.front_cursor;
        let bc = self.back_cursor;

        self.iterator.get_or_compute(
            |xs| { range_utils::make_to_many_key_range(open_parent(xs), fc, bc) },
            |range| { body(range) },
        )
    }

    pub(crate) fn next<V: IdLike>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>
    ) -> Option<K> {
        if self.done { return None; }

        let k = self.reconstitute(open_parent, |iter| { iter.next().map(|(k, _)| *k) });
        self.front_cursor = k; 
        if k == None { self.done = true; }
        k
    }

    pub(crate) fn next_back<V: IdLike>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>
    ) -> Option<K> { 
        if self.done { return None; }

        let k = self.reconstitute(open_parent, |iter| { iter.next_back().map(|(k, _)| *k) });
        self.back_cursor = k; 
        if k == None { self.done = true; }
        k
    }
}

pub(crate) struct ToManyKeyValueIterator<'a, Parent, K: IdLike, V: IdLike> {
    iterator: InteriorSetRange<'a, Parent, (K, V)>,

    // unlike cursors, these aren't skipped
    front_element: Option<(K, V)>,
    back_element: Option<(K, V)>,

    front_cursor: Option<(K, V)>,
    back_cursor: Option<(K, V)>,
    done: bool,
}

impl<'a, Parent, K: IdLike, V: IdLike> ToManyKeyValueIterator<'a, Parent, K, V> {
    pub(crate) fn new(
        iterator: InteriorSetRange<'a, Parent, (K, V)>, 
        front_element: Option<(K, V)>,
        back_element: Option<(K, V)>,
    ) -> ToManyKeyValueIterator<'a, Parent, K, V> {
        ToManyKeyValueIterator {
            iterator,

            front_element, back_element,
            front_cursor: None,
            back_cursor: None,
            done: false,
        }
    }

    fn reconstitute<T>(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>,
        body: impl FnOnce(&mut btree_set::Range<'_, (K, V)>) -> T
    ) -> T {
        let fc = self.front_cursor;
        let bc = self.back_cursor;
        let fe = self.front_element;
        let be = self.back_element;

        self.iterator.get_or_compute(
            |xs| { range_utils::make_to_many_key_value_range(open_parent(xs), fc, bc, fe, be) },
            |range| { body(range) },
        )
    }

    pub(crate) fn next(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>
    ) -> Option<(K, V)> {
        if self.done { return None; }

        let k = self.reconstitute(open_parent, |iter| { iter.next().map(|k| *k) });
        self.front_cursor = k; 
        if k == None { self.done = true; }
        k
    }

    pub(crate) fn next_back(
        &mut self, 
        open_parent: impl FnOnce(&Parent) -> &ToMany<K, V>
    ) -> Option<(K, V)> { 
        if self.done { return None; }

        let k = self.reconstitute(open_parent, |iter| { iter.next_back().map(|k| *k) });
        self.back_cursor = k; 
        if k == None { self.done = true; }
        k
    }
}