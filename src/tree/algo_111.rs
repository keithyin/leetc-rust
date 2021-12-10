use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;

pub fn min_depth_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut min_depth = 0;
    if let Some(node) = root {
        let mut min_right_depth = 0;
        let mut min_left_depth = 0;
        if let Some(ref left_node) = node.as_ref().borrow().left {
            min_left_depth = min_depth_in_tree(Some(Rc::clone(left_node)));
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            min_right_depth = min_depth_in_tree(Some(Rc::clone(right_node)));
        }
        min_depth = if min_left_depth > min_right_depth {
            min_right_depth
        } else {
            min_left_depth
        };
        min_depth += 1;
    } else {
        0;
    };
    min_depth
}

#[cfg(test)]
mod test {
    use crate::tree::tree_traverse::get_tree;
    use crate::tree::algo_111::min_depth_in_tree;

    #[test]
    fn test_min_depth() {
        let tree = get_tree();
        assert_eq!(min_depth_in_tree(tree), 2);
    }
}