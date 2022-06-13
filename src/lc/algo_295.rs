use std::cmp::Reverse;
use std::collections::binary_heap;

struct MedianFinder {
    max_heap: binary_heap::BinaryHeap<i32>,
    min_heap: binary_heap::BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        MedianFinder{
            max_heap: binary_heap::BinaryHeap::new(),
            min_heap: binary_heap::BinaryHeap::new()}
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.len() == 0 {
            self.max_heap.push(num);
            return;
        }

        if num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        if (self.max_heap.len() as i32 - self.min_heap.len() as i32) >= 2 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        } else if (self.min_heap.len() as i32 - self.max_heap.len() as i32) >= 2 {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            return (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0;
        }

        return if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            self.min_heap.peek().unwrap().0 as f64
        }

    }
}
