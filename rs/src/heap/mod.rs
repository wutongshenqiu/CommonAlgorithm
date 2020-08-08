use std::hash::Hash;
use std::cmp::Ordering;
use std::fmt;


pub struct HeapElement<K: Ord, V: Hash> {
    key: K,
    track_id: V,
}

impl<K: Ord, V: Hash> HeapElement<K, V> {
    pub fn new(key: K, track_id: V) -> HeapElement<K, V> {
        HeapElement {
            key,
            track_id
        }
    }
}

impl<K: Ord, V: Hash> PartialEq for HeapElement<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V: Hash> Eq for HeapElement<K, V> {}

impl<K: Ord, V: Hash> PartialOrd for HeapElement<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V: Hash> Ord for HeapElement<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord + fmt::Display, V: Hash + fmt::Display> fmt::Display for HeapElement<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key: {}, track_id: {}", self.key, self.track_id)
    }
}



pub trait HeapMethod<K: Ord, V: Hash> {
    // get certain element with track_id(key of map)
    fn get_element(&self, track_id: usize) -> Option<HeapElement<K, V>>;
    // add element to heap
    fn push(&self, element: HeapElement<K, V>);
    // get and delete the top(min/max) element from heap
    fn pop(&self) -> Option<HeapElement<K, V>>;
    // decrease key of existing element
    fn decrease_key(&self, element: HeapElement<K, V>);
}