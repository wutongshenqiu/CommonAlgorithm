use std::hash::Hash;
use std::cmp::Ordering;
use std::fmt;

mod binary_heap;

pub use self::binary_heap::BinaryMinHeap;

#[derive(Debug)]
pub struct HeapElement<K: Ord, V: Hash + Eq + Copy> {
    key: K,
    track_id: V,
}

impl<K: Ord, V: Hash + Eq + Copy> HeapElement<K, V> {
    pub fn new(key: K, track_id: V) -> HeapElement<K, V> {
        HeapElement {
            key,
            track_id
        }
    }

    pub fn reset_key(&mut self, key: K) {
        self.key = key;
    }
}

impl<K: Ord, V: Hash + Eq + Copy> PartialEq for HeapElement<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V: Hash + Eq + Copy> Eq for HeapElement<K, V> {}

impl<K: Ord, V: Hash + Eq + Copy> PartialOrd for HeapElement<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V: Hash + Eq + Copy> Ord for HeapElement<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord + fmt::Display, V: Hash + Eq + Copy + fmt::Display> fmt::Display for HeapElement<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key: {}, track_id: {}", self.key, self.track_id)
    }
}



pub trait HeapMethod<K: Ord, V: Hash + Eq + Copy> {
    // create heap
    fn new() -> Self;
    // create heap with specific capacity
    fn with_capacity(capacity: usize) -> Self;
    // get certain element with track_id(key of map)
    fn get_element(&self, track_id: V) -> Option<&HeapElement<K, V>>;
    // get mutable certain element with track_id(key of map)
    fn get_mut_element(&mut self, track_id: V) -> Option<&mut HeapElement<K, V>>;
    // add element to heap
    fn push(&mut self, element: HeapElement<K, V>);
    // get and delete the top(min/max) element from heap
    fn pop(&mut self) -> Option<HeapElement<K, V>>;
    // decrease key of existing element
    fn decrease_key(&mut self, key: K, track_id: V);
}
