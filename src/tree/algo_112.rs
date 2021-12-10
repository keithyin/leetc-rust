use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;

pub fn target_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target: i32) -> bool {

    if let Some(node) = root {
        target -= node.as_ref().borrow().val;

        if node.as_ref().borrow().right == None && node.as_ref().borrow().left == None {
            if target == 0 {
                return true;
            } else {
                return false;
            }
        }
        let mut left_valid = false;
        let mut right_valid = false;
        if let Some(ref left_node) = node.as_ref().borrow().left {
            left_valid = target_sum(Some(Rc::clone(left_node)), target);
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            right_valid = target_sum(Some(Rc::clone(right_node)), target);
        }
        return left_valid || right_valid;
    }
    false
}

#[cfg(test)]
mod test {
    use crate::tree::tree_traverse::get_tree;
    use crate::tree::algo_112::target_sum;

    #[test]
    fn test_target_sum_valid() {
        let tree = get_tree();
        // println!("{}", target_sum(tree, 51));
        assert!(target_sum(tree, 51));
    }
}