use std::cell::{Ref, RefCell};
use std::cmp::min;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn min_depth_core(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_depth = min_depth_core(&root.as_ref().unwrap().borrow().left);
    let right_depth = min_depth_core(&root.as_ref().unwrap().borrow().right);
    if left_depth == 0 {
        return 1 + right_depth;
    } else if right_depth == 0 {
        return 1 + left_depth;
    } else {
        return 1 + min(left_depth, right_depth);
    }
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    min_depth_core(&root)
}