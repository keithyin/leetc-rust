use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;
use std::rc::Rc;

pub fn is_symmetric_tree(root1: &Option<Rc<RefCell<TreeNode>>>,
                         root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root1 == &None && root2 == &None {
        return true;
    }
    return if let (Some(node1), Some(node2)) = (root1, root2) {
        if node1.as_ref().borrow().val == node2.as_ref().borrow().val {
            is_symmetric_tree(&node1.as_ref().borrow().left, &node2.as_ref().borrow().right)
                && is_symmetric_tree(&node1.as_ref().borrow().right, &node2.as_ref().borrow().left)
        } else {
            false
        }
    } else {
        false
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree::build_tree::{get_symmetric_tree, get_tree1};

    #[test]
    fn test_is_symmetric_tree() {
        let root = get_symmetric_tree();
        assert!(is_symmetric_tree(&root, &root));
        let root2 = get_tree1();
        assert!(!is_symmetric_tree(&root2, &root2));
    }
}