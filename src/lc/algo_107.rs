use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut first_queue = LinkedList::new();
    let mut second_queue = LinkedList::new();
    first_queue.push_back(root);
    let mut result = vec![];
    while !first_queue.is_empty() || !second_queue.is_empty() {
        let mut cur_level = vec![];
        while !first_queue.is_empty() {
            if let Some(node) = first_queue.pop_front() {
                cur_level.push(node.as_ref().unwrap().borrow().val);
                if let Some(ref left_node) = node.as_ref().unwrap().borrow().left {
                    second_queue.push_back(Some(Rc::clone(left_node)));
                }
                if let Some(ref right_node) = node.as_ref().unwrap().borrow().right {
                    second_queue.push_back(Some(Rc::clone(right_node)));
                }
            }

        }
        if cur_level.len() > 0 {
            result.push(cur_level);
            continue;
        }

        while !second_queue.is_empty() {
            if let Some(node) = second_queue.pop_front() {
                cur_level.push(node.as_ref().unwrap().borrow().val);
                if let Some(ref left_node) = node.as_ref().unwrap().borrow().left {
                    first_queue.push_back(Some(Rc::clone(left_node)));
                }
                if let Some(ref right_node) = node.as_ref().unwrap().borrow().right {
                    first_queue.push_back(Some(Rc::clone(right_node)));
                }
            }
        }

        if cur_level.len() > 0 {
            result.push(cur_level);
            continue;
        }
    }

    let mut res = vec![];
    while let Some(v) = result.pop() {
        res.push(v);
    }
    res

}