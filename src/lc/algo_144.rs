use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn preorder_traversal_core(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    result.push(root.as_ref().unwrap().borrow().val);
    preorder_traversal_core(&root.as_ref().unwrap().borrow().left, result);
    preorder_traversal_core(&root.as_ref().unwrap().borrow().right, result);
}

pub fn preorder_traversal_stack(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    if root.is_none() {
        return vec![];
    }
    let mut stack = LinkedList::new();
    let mut result = vec![];
    result.push(root.as_ref().unwrap().borrow().val);
    stack.push_back(root.as_ref().unwrap());

    while !stack.is_empty() {
        let mut next = &stack.back().unwrap().borrow().left;
        if next.is_none() {
            let right_node = &stack.pop_back().unwrap().borrow().right;
            result.push(right_node.as_ref().unwrap().borrow().val);
            stack.push_back(right_node.as_ref().unwrap());
        } else {
            result.push(next.as_ref().unwrap().borrow().val);
            stack.push_back(next.as_ref().unwrap());
        }
    }
    result
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    preorder_traversal_stack(&root)
}