use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use tracing::trace;

pub trait VecManipulation {
    fn dedup_by_key<T, K, F>(items: Vec<T>, mut key: F) -> Vec<T>
    where
        T: Debug,
        K: Eq + Hash,
        F: FnMut(&T) -> K,
    {
        let mut seen = HashSet::new();
        let mut out = Vec::with_capacity(items.len());

        for item in items {
            let k = key(&item);
            if seen.insert(k) {
                out.push(item);
            } else {
                trace!("Skipping duplicate: {:?}", item);
            }
        }

        out
    }
}
