use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::tree_traverse::TreeNode;

pub fn tree_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left_depth = tree_depth(&node.as_ref().borrow().left);
        let right_depth = tree_depth(&node.as_ref().borrow().right);
        return if left_depth > right_depth {
            left_depth + 1
        }else {
            right_depth + 1
        };
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree::build_tree::get_tree1;

    #[test]
    fn test_tree_depth() {
        let root = get_tree1();
        assert_eq!(tree_depth(&root), 3);
    }
}