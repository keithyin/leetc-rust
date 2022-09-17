use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn has_path_sum_core(root: &Option<Rc<RefCell<TreeNode>>>, mut residual: i32) -> bool {

    if root.is_none() {
        return false;
    }

    residual = residual - root.as_ref().unwrap().borrow().val;
    if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
        return true;
    }
    let mut left_found = has_path_sum_core(&root.as_ref().unwrap().borrow().left, residual);
    if left_found {
        return left_found;
    }
    let mut right_found = has_path_sum_core(&root.as_ref().unwrap().borrow().right, residual);
    left_found || right_found
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }


    has_path_sum_core(&root, target_sum)
}