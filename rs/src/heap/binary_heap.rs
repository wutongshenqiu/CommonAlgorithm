use super::{HeapElement, HeapMethod};

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;


pub struct BinaryMinHeap<K: Ord, V: Hash + Eq + Copy> {
    data: Vec<HeapElement<K, V>>,
    map: HashMap<V, usize>,
}

impl<K: Ord, V: Hash + Eq + Copy> HeapMethod<K, V> for BinaryMinHeap<K, V> {
    fn new() -> Self {
        BinaryMinHeap {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        BinaryMinHeap {
            data: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }

    fn get_element(&self, track_id: V) -> Option<&HeapElement<K, V>> {
        match self.map.get(&track_id) {
            Some(index) => Some(&self.data[*index]),
            None => None,
        }
    }

    fn get_mut_element(&mut self, track_id: V) -> Option<&mut HeapElement<K, V>> {
        match self.map.get(&track_id) {
            Some(index) => Some(&mut self.data[*index]),
            None => None,
        }
    }

    fn pop(&mut self) -> Option<HeapElement<K, V>> {
        match self.data.len() {
            0 => None,
            1 => {
                let pop_element = self.data.pop().unwrap();
                self.map.remove(&pop_element.track_id);
                Some(pop_element)
            }
            _ => {
                let last_element_index = self.data.len() - 1;
                self.data.swap(0, last_element_index);
                self.map.insert(self.data[0].track_id, 0);
                
                let pop_element = self.data.pop().unwrap();
                self.map.remove(&pop_element.track_id);
                
                self.sift_down(0);

                Some(pop_element)
            }
        }
    }

    fn push(&mut self, element: HeapElement<K, V>) {
        if self.map.contains_key(&element.track_id) {
            self.decrease_key(element.key, element.track_id);
        }
        else {
            let push_element_index = self.data.len();
            self.map.insert(element.track_id, push_element_index);
            self.data.push(element);

            self.sift_up(push_element_index);
        }
    }

    // https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
    // can not borrow mutable and immutable at the same time
    fn decrease_key(&mut self, key: K, track_id: V) {
        if let Some(old_element_index) = self.map.get(&track_id) {
            // todo
            // still not very understand here
            let old_element_index = old_element_index.to_owned();
            let old_element = &mut self.data[old_element_index];
            if key < old_element.key {
                old_element.reset_key(key);
                self.sift_up(old_element_index);
            }
        }        
    }
}

impl<K: Ord, V: Hash + Eq + Copy> BinaryMinHeap<K, V> {
    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_element_index = (index - 1) / 2;
            if self.data[index] < self.data[parent_element_index] {
                self.swap(index, parent_element_index);
                index = parent_element_index;
            }
            else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let mut left_child_index = 2 * index + 1;
        let mut right_child_index = 2 * index + 2;

        while left_child_index < self.data.len() {
            // if existing right child
            if right_child_index < self.data.len() {
                if self.data[left_child_index] < self.data[right_child_index] {
                    if self.data[index] <= self.data[left_child_index] {
                        break;
                    }
                    else {
                        self.swap(index, left_child_index);
                        index = left_child_index;
                    }
                }
                else {
                    if self.data[index] <= self.data[right_child_index] {
                        break;
                    }
                    else {
                        self.swap(index, right_child_index);
                        index = right_child_index;
                    }
                }
            }
            else {
                if self.data[index] <= self.data[left_child_index] {
                    break;
                }
                else {
                    self.swap(index, left_child_index);
                    index = left_child_index;
                }
            }

            left_child_index = 2 * index + 1;
            right_child_index = 2 * index + 2;
        }
    }

    fn swap(&mut self, index1: usize, index2: usize) {
        let track_id1 = self.data[index1].track_id;
        let track_id2 = self.data[index2].track_id;
        
        self.map.insert(track_id1, index2);
        self.map.insert(track_id2, index1);

        self.data.swap(index1, index2);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_heap() {
        let heap = BinaryMinHeap::new();
        test_heap(heap);
    }

    fn test_heap(mut heap: impl HeapMethod<usize, usize>) {
        heap.push(HeapElement::new(5, 1));
        heap.push(HeapElement::new(3, 1));
        heap.push(HeapElement::new(7, 2));
        heap.push(HeapElement::new(12, 3));
        heap.push(HeapElement::new(11, 4));
        heap.push(HeapElement::new(20, 5));
        heap.push(HeapElement::new(16, 6));
        heap.push(HeapElement::new(9, 7));
        heap.push(HeapElement::new(12, 7));
        heap.push(HeapElement::new(4, 7));

        assert_eq!(heap.get_element(1).unwrap(), &HeapElement::new(3, 1));
        assert_eq!(heap.get_element(2).unwrap(), &HeapElement::new(7, 2));
        assert_eq!(heap.get_element(3).unwrap(), &HeapElement::new(12, 3));
        assert_eq!(heap.get_element(4).unwrap(), &HeapElement::new(11, 4));
        assert_eq!(heap.get_element(5).unwrap(), &HeapElement::new(20, 5));
        assert_eq!(heap.get_element(6).unwrap(), &HeapElement::new(16, 6));
        assert_eq!(heap.get_element(7).unwrap(), &HeapElement::new(4, 7));
        assert_eq!(heap.get_element(8), None);
        
        assert_eq!(heap.pop().unwrap(), HeapElement::new(3, 1));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(4, 7));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(7, 2));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(11, 4));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(12, 3));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(16, 6));
        assert_eq!(heap.pop().unwrap(), HeapElement::new(20, 5));
        assert_eq!(heap.pop(), None);
    }
}