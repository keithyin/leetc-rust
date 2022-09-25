use std::cell::{Ref, RefCell};
use std::collections::LinkedList;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut queue1 = LinkedList::new();
    let mut queue2 = LinkedList::new();
    queue1.push_back(Rc::clone(root.as_ref().unwrap()));

    let mut result = vec![];

    while !queue1.is_empty() || !queue2.is_empty() {
        while let Some(node) = queue1.pop_front(){
            if queue1.is_empty() {
                result.push(node.borrow().val);
            }
            if let Some(ref right_node) = node.as_ref().borrow().right {
                queue2.push_back(Rc::clone(right_node));
            }

            if let Some(ref left_node) = node.as_ref().borrow().left {
                queue2.push_back(Rc::clone(left_node));
            }
        }

        while let Some(ref node) = queue2.pop_front() {
            if queue2.is_empty() {
                result.push(node.as_ref().borrow().val);
            }

            if let Some(ref right_node) = node.as_ref().borrow().right {
                queue1.push_back(Rc::clone(right_node));
            }

            if let Some(ref left_node) = node.as_ref().borrow().left {
                queue1.push_back(Rc::clone(left_node));
            }
        }
    }
    result
}