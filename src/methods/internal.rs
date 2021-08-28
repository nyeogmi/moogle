use crate::id::IdLike;

use super::ViewSet;

pub(crate) trait EvictSet<'a, K: IdLike, V: IdLike>: ViewSet<'a, V> {
    fn insert(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;  // return the evicted item if one was evicted
    fn remove(&mut self, v: V, on_evict: impl FnOnce(K, V)) -> Option<V>;
} 