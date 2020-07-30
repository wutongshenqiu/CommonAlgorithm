pub fn heap_sort<T: Ord + Copy>(l: Vec<T>) -> Vec<T>{
    let mut sorted_list: Vec<T> = Vec::with_capacity(l.capacity());

    let mut small_heap = SmallHeap::new(l);

    while !small_heap.is_empty() {
        sorted_list.push(small_heap.pop_top());
    }

    sorted_list
}

pub struct SmallHeap<T: Ord + Copy> {
    list: Vec<T>,
}

impl<T: Ord + Copy> SmallHeap<T> {
    pub fn new(list: Vec<T>) -> SmallHeap<T> {
        let mut small_heap = SmallHeap {
            list
        };

        small_heap.make_heap();
        small_heap
    }

    pub fn sift_down(&mut self, index: usize) {
        let mut j = index + 1;

        while 2 * j <= self.list.len() {
            let mut i = 2 * j;
            if i < self.list.len() && self.list[i] < self.list[i-1] {
                i += 1;
            }
            if self.list[i-1] < self.list[j-1] {
                self.list.swap(i-1, j-1);
                j = i;
            }
            else {
                break;
            }
        }
    }

    fn make_heap(&mut self) {
        for i in (0..self.list.len() / 2).rev() {
            self.sift_down(i);
        }
    }

    pub fn pop_top(&mut self) -> T {
        match self.list.len() {
            0 => panic!("list is empty!"),
            1 => self.list.pop().unwrap(),
            _ => {
                let top_element = self.list[0];
                self.list[0] = self.list.pop().unwrap();
                self.sift_down(0);
                top_element
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }
}




