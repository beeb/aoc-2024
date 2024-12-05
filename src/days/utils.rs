use std::ops::{Deref, DerefMut};

use ahash::{AHashSet, RandomState};
use winnow::stream::Accumulate;

/// Custom type for AHashSet because we can't implement [`Accumulate`] for foreign types
#[derive(Debug, Clone)]
pub struct MyHashSet<K>(AHashSet<K, RandomState>);

impl<T> Deref for MyHashSet<T> {
    type Target = AHashSet<T, RandomState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyHashSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> Accumulate<K> for MyHashSet<K>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    #[inline(always)]
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => MyHashSet(AHashSet::with_capacity(capacity)),
            None => MyHashSet(AHashSet::new()),
        }
    }
    #[inline(always)]
    fn accumulate(&mut self, key: K) {
        self.0.insert(key);
    }
}
