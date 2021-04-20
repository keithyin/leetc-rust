use crate::tree::tree_traverse;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use crate::tree::tree_traverse::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool{
    let (_, _, valid, _) = is_valid_bst_core(&root);
    return valid;
}

pub fn is_valid_bst_core(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, bool, bool){
    if let Some(node) = root {
        let (left_min, left_max, left_valid, left_is_none_node) = is_valid_bst_core(&(node.as_ref().borrow().left));
        if !left_valid {
            return (0, 0, false, false);
        }
        let (right_min, right_max, right_valid, right_is_non_node) = is_valid_bst_core(&(node.as_ref().borrow().right));
        if !right_valid {
            return (0, 0, false, false);
        }

        let cur_node_val = node.as_ref().borrow().val;
        let left_valid = left_is_none_node || cur_node_val > left_max;
        let right_valid = right_is_non_node || cur_node_val < right_min;

        let mut min_val = left_min;
        if left_is_none_node {
            min_val = cur_node_val;
        }
        let mut max_val = right_max;
        if right_is_non_node {
            max_val = cur_node_val;
        }
        return (min_val, max_val, left_valid && right_valid, false);

    };
    return (i32::MIN, i32::MAX, true, true);
}
