use std::mem;
use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedListNode {
    pub val: i32,
    pub next: Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    pub fn new(val: i32) -> Self {
        LinkedListNode {
            val,
            next: None,
        }
    }
}

#[derive(Debug)]
struct Heap {
    heap: Vec<Box<LinkedListNode>>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            heap: vec![Box::new(LinkedListNode::new(-1))],
        }
    }

    pub fn push(&mut self, item: Box<LinkedListNode>) {
        self.heap.push(item);
        self.swim();
    }

    fn swim(&mut self) {
        let num_item = self.heap.len();
        if num_item == 1 {
            return;
        }
        let mut cur_item_idx = num_item - 1;
        let mut father_idx = cur_item_idx / 2;
        while father_idx > 0 && self.heap[father_idx].val > self.heap[cur_item_idx].val {
            let first_son_of_father = father_idx * 2;
            let second_son_of_father = first_son_of_father + 1;
            let mut smaller_son = first_son_of_father;
            if second_son_of_father < self.heap.len() {
                smaller_son = if self.heap[first_son_of_father].val < self.heap[second_son_of_father].val {
                    first_son_of_father
                } else {
                    second_son_of_father
                };
            }
            self.heap.swap(father_idx, smaller_son);
            cur_item_idx = father_idx;
            father_idx = cur_item_idx / 2;
        }
    }

    fn sink(&mut self) {
        let mut father_idx = 1 as usize;
        let length = self.heap.len();
        while father_idx < length && 2 * father_idx < length{
            let first_son_idx = 2 * father_idx;
            let second_son_idx = first_son_idx + 1;
            let mut smaller_son_idx = first_son_idx;
            if second_son_idx < length && self.heap[first_son_idx].val > self.heap[second_son_idx].val{
                smaller_son_idx = second_son_idx;
            }
            if self.heap[father_idx].val < self.heap[smaller_son_idx].val {
                break;
            }
            self.heap.swap(father_idx, smaller_son_idx);
            father_idx = smaller_son_idx;
        }
    }

    pub fn pop(&mut self) -> Option<Box<LinkedListNode>>{
        if self.heap.len() <= 1 {
            None
        } else {
            let length = self.heap.len();
            self.heap.swap(1, length - 1);
            let result = self.heap.pop();
            self.sink();
            result
        }
    }

    pub fn top(&self) -> Option<&Box<LinkedListNode>> {
        if self.heap.len() <= 1 {
            None
        } else {
            Some(&self.heap[1])
        }
    }
}

pub fn merge_k_ordered_linked_list(k_list: Vec<Option<Box<LinkedListNode>>>) {
    let mut new_head = Box::new(0);
    Rc::from(new_head);

}

#[cfg(test)]
mod test {
    use std::mem;
    use crate::link_list::algo_23::{Heap, LinkedListNode};

    #[test]
    fn test_swap() {
        let mut a = vec![1, 2];
        a.swap(0, 1);
        println!("{:?}", a);
    }

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        heap.push(Box::new(LinkedListNode::new(5)));
        heap.push(Box::new(LinkedListNode::new(2)));
        heap.push(Box::new(LinkedListNode::new(1)));
        println!("{:?}", heap);
        println!("{:?}", heap.pop());
        println!("{:?}", heap);
    }
}