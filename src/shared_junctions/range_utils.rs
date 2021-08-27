use std::collections::{btree_map, BTreeMap, btree_set, BTreeSet};

use crate::structures::ToSet;
use crate::keybound::Id;

#[inline]
pub(crate) fn make_toset_range<K: Id, V: Id>(
    vs: &ToSet<K, V>, front_cursor: Option<K>, back_cursor: Option<K>,
) -> btree_map::Range<'_, K, BTreeSet<V>> {
    match (front_cursor, back_cursor) {
        (Some(f), Some(b)) => {
            let mut l = vs.range(f..=b);
            l.next();
            l.next_back();
            l
        },
        (Some(f), None) => {
            let mut l = vs.range(f..);
            l.next();
            l
        }
        (None, Some(b)) => {
            let mut l = vs.range(..=b);
            l.next_back();
            l
        }
        (None, None) => vs.range(..)
    }
}

#[inline]
pub fn make_btreemap_range<K: Copy+Ord, V>(
    vs: &BTreeMap<K, V>, front_cursor: Option<K>, back_cursor: Option<K>,
) -> btree_map::Range<'_, K, V> {
    match (front_cursor, back_cursor) {
        (Some(f), Some(b)) => {
            let mut l = vs.range(f..=b);
            l.next();
            l.next_back();
            l
        },
        (Some(f), None) => {
            let mut l = vs.range(f..);
            l.next();
            l
        }
        (None, Some(b)) => {
            let mut l = vs.range(..=b);
            l.next_back();
            l
        }
        (None, None) => vs.range(..)
    }
}

#[inline]
pub fn make_btreeset_range<K: Copy+Ord>(
    vs: &BTreeSet<K>, front_cursor: Option<K>, back_cursor: Option<K>,
) -> btree_set::Range<'_, K> {
    match (front_cursor, back_cursor) {
        (Some(f), Some(b)) => {
            let mut l = vs.range(f..=b);
            l.next();
            l.next_back();
            l
        },
        (Some(f), None) => {
            let mut l = vs.range(f..);
            l.next();
            l
        }
        (None, Some(b)) => {
            let mut l = vs.range(..=b);
            l.next_back();
            l
        }
        (None, None) => vs.range(..)
    }
}