use std::collections::HashSet;
use std::hash::Hash;

pub fn set<T: Clone + Hash + Eq + PartialEq>(strings: &[T]) -> HashSet<T> {
    strings.iter().cloned().collect()
}
