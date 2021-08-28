use std::collections::{btree_map, BTreeMap, btree_set};

use crate::structures::ToSet;
use crate::id::IdLike;

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
    vs: &ToSet<K, V>, 
    front_cursor: Option<(K, V)>, back_cursor: Option<(K, V)>,

    // nyeo note: these are like cursors but don't need to be advanced past
    front_element: Option<(K, V)>, back_element: Option<(K, V)>,  
) -> btree_set::Range<'_, (K, V)> {
    let mut l = match (front_cursor.or(front_element), back_cursor.or(back_element) ) {
        (Some(f), Some(b)) => { vs.element_subrange(f..=b) }
        (Some(f), None) => { vs.element_subrange(f..) }
        (None, Some(b)) => { vs.element_subrange(..=b) }
        (None, None) => { vs.element_subrange(..) }
    };

    match (front_cursor, back_cursor) {
        (Some(_), Some(_)) => { l.next(); l.next_back(); },
        (Some(_), None) => { l.next(); }
        (None, Some(_)) => { l.next_back(); }
        (None, None) => {}
    };

    l
}

#[inline]
pub fn make_map_range<K: Copy+Ord, V>(
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