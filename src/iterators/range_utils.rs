use std::collections::{btree_map, BTreeMap, btree_set};

use crate::structures::ToSet;
use crate::id::IdLike;
use crate::structures::VSet;

#[inline]
pub(crate) fn make_toset_key_range<K: IdLike, V: IdLike>(
    vs: &ToSet<K, V>, front_cursor: Option<K>, back_cursor: Option<K>,
) -> btree_set::Range<'_, K> {
    match (front_cursor, back_cursor) {
        (Some(f), Some(b)) => {
            let mut l = vs.key_subrange(f..=b);
            l.next();
            l.next_back();
            l
        },
        (Some(f), None) => {
            let mut l = vs.key_subrange(f..);
            l.next();
            l
        }
        (None, Some(b)) => {
            let mut l = vs.key_subrange(..=b);
            l.next_back();
            l
        }
        (None, None) => vs.key_subrange(..)
    }
}

#[inline]
pub(crate) fn make_toset_key_value_range<K: IdLike, V: IdLike>(
    vs: &ToSet<K, V>, front_cursor: Option<(K, V)>, back_cursor: Option<(K, V)>,
) -> btree_set::Range<'_, (K, V)> {
    match (front_cursor, back_cursor) {
        (Some(f), Some(b)) => {
            let mut l = vs.element_subrange(f..=b);
            l.next();
            l.next_back();
            l
        },
        (Some(f), None) => {
            let mut l = vs.element_subrange(f..);
            l.next();
            l
        }
        (None, Some(b)) => {
            let mut l = vs.element_subrange(..=b);
            l.next_back();
            l
        }
        (None, None) => vs.element_subrange(..)
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
pub(crate) fn make_vset_range<'a, K: IdLike, V: IdLike>(
    vs: &VSet<'a, K, V>, front_cursor: Option<V>, back_cursor: Option<V>,
) -> btree_set::Range<'a, (K, V)> {
    let mut l = vs.range(front_cursor, back_cursor);

    match (front_cursor, back_cursor) {
        (Some(_), Some(_)) => { l.next(); l.next_back(); },
        (Some(_), None) => { l.next(); }
        (None, Some(_)) => { l.next_back(); }
        (None, None) => {}
    };

    l
}