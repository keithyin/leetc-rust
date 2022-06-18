use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    let mut res_val = 0;
    kth_smallest_core(&root, &mut k, &mut res_val);
    res_val

}

fn kth_smallest_core(root: &Option<Rc<RefCell<TreeNode>>>, counter: &mut i32, res_val: &mut i32) {
    if let Some(cur) = root {
        if let Some(ref left_node) = cur.as_ref().borrow().left {
            kth_smallest_core(&Some(Rc::clone(left_node)), counter, res_val);
        }

        *counter -= 1;
        if *counter == 0 {
            *res_val = cur.as_ref().borrow().val;
            return;
        }
        if let Some(ref right_node) = cur.as_ref().borrow().right {
            kth_smallest_core(&Some(Rc::clone(right_node)), counter, res_val);
        }
    }
}