use std::rc::Rc;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

#[derive(Debug)]
pub struct LinkedListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<LinkedListNode>>>,
}

impl LinkedListNode {
    pub fn print(&self) {
        print!("{} ", self.val);
        let mut cursor = if let Some(ref node) = self.next{
            Some(Rc::clone(node))
        } else {
            None
        };

        loop {
            if let Some(ref node) = cursor {
                print!("{} ", node.as_ref().borrow().val);
                let next_node = if let Some(ref node) = node.as_ref().borrow().next {
                    Some(Rc::clone(node))
                } else {
                    None
                };
                cursor = next_node;  // Attention
            } else {
                break;
            }
        }
        println!();
    }

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
    heap: Vec<Rc<RefCell<LinkedListNode>>>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            heap: vec![Rc::new(RefCell::new(LinkedListNode::new(-1)))],
        }
    }

    pub fn push(&mut self, item: Rc<RefCell<LinkedListNode>>) {
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
        while father_idx > 0 && self.heap[father_idx].as_ref().borrow().val > self.heap[cur_item_idx].as_ref().borrow().val {
            let first_son_of_father = father_idx * 2;
            let second_son_of_father = first_son_of_father + 1;
            let mut smaller_son = first_son_of_father;
            if second_son_of_father < self.heap.len() {
                smaller_son = if self.heap[first_son_of_father].as_ref().borrow().val < self.heap[second_son_of_father].as_ref().borrow().val {
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
            if second_son_idx < length && self.heap[first_son_idx].as_ref().borrow().val > self.heap[second_son_idx].as_ref().borrow().val{
                smaller_son_idx = second_son_idx;
            }
            if self.heap[father_idx].as_ref().borrow().val < self.heap[smaller_son_idx].as_ref().borrow().val {
                break;
            }
            self.heap.swap(father_idx, smaller_son_idx);
            father_idx = smaller_son_idx;
        }
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<LinkedListNode>>>{
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

    pub fn top(&self) -> Option<Rc<RefCell<LinkedListNode>>> {
        if self.heap.len() <= 1 {
            None
        } else {
            Some(Rc::clone(&self.heap[1]))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() > 1
    }
}

pub fn merge_k_ordered_linked_list(mut k_list: Vec<Option<Rc<RefCell<LinkedListNode>>>>) -> Option<Rc<RefCell<LinkedListNode>>> {
    let mut new_head = Rc::new(RefCell::new(LinkedListNode::new(0)));
    let mut cursor = Rc::clone(&new_head);

    let mut heap = Heap::new();
    while let Some(Some(node)) = k_list.pop() {
        heap.push(node)
    }
    while let Some(node) = heap.pop() {
        cursor.as_ref().borrow_mut().next = Some(Rc::clone(&node));
        if let Some(ref next_node) = node.as_ref().borrow().next {
            heap.push(Rc::clone(next_node));
        }
        cursor = node;
    }
    return if let Some(ref node) = new_head.as_ref().borrow().next {
        Some(Rc::clone(node))
    } else {
        None
    };

}

pub fn build_linked_list(mut vals: Vec<i32>) -> Option<Rc<RefCell<LinkedListNode>>>{
    vals.sort();
    let mut head = Rc::new(RefCell::new(LinkedListNode::new(-1)));
    let mut cursor = Rc::clone(&head);
    for v in vals.iter() {
        let node = Rc::new(RefCell::new(LinkedListNode::new(*v)));
        cursor.as_ref().borrow_mut().next = Some(Rc::clone(&node));
        cursor = node;
    }
    return if let Some(ref actual_head) = head.as_ref().borrow().next {
        Some(Rc::clone(actual_head))
    } else {
        None
    };

}


#[cfg(test)]
mod test {
    use crate::linked_list::algo_23::{Heap, LinkedListNode, build_linked_list, merge_k_ordered_linked_list};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_swap() {
        let mut a = vec![1, 2];
        a.swap(0, 1);
        println!("{:?}", a);
    }

    #[test]
    fn test_build_linked_list() {
        let linked_list = build_linked_list(vec![2, 3, 9, 10]);
        if let Some(ref head) = linked_list {
            head.as_ref().borrow().print();
        }
    }

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        heap.push(Rc::new(RefCell::new(LinkedListNode::new(5))));
        heap.push(Rc::new(RefCell::new(LinkedListNode::new(2))));
        heap.push(Rc::new(RefCell::new(LinkedListNode::new(1))));
        println!("{:?}", heap);
        println!("{:?}", heap.pop());
        println!("{:?}", heap);
    }

    #[test]
    fn test_merge_k_ordered_linked_list() {
        let linked_list1 = build_linked_list(vec![1, 4, 9]);
        let linked_list2 = build_linked_list(vec![2, 3, 6]);
        let linked_list3 = build_linked_list(vec![4, 5, 8]);
        let k_list = vec![linked_list1, linked_list2, linked_list3];
        let res_list = merge_k_ordered_linked_list(k_list);
        if let Some(ref res) = res_list {
            res.as_ref().borrow().print();
        }

    }
}