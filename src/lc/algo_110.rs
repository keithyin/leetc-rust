use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn is_balanced_core(root: &Option<Rc<RefCell<TreeNode>>>, balance: &mut bool) -> i32{
    if root.is_none() {
        return 0;
    }

    if !*balance {
        return 0;
    }

    let mut left_height = is_balanced_core(&root.as_ref().unwrap().borrow().left, balance);
    let mut right_height = is_balanced_core(&root.as_ref().unwrap().borrow().right, balance);

    if (left_height - right_height).abs() > 1 {
        *balance = false;
    }

    return 1 + max(left_height, right_height);
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut balance = true;
    is_balanced_core(&root, &mut balance);
    balance
}