use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;
use std::rc::Rc;
use std::borrow::Borrow;

pub fn is_same_tree(root1: &Option<Rc<RefCell<TreeNode>>>,
                    root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root1 == &None && root2 == &None {
        return true;
    }

    if let (Some(node1), Some(node2)) = (root1, root2){
        if node1.as_ref().borrow().val == node2.as_ref().borrow().val {
            return is_same_tree(&node1.as_ref().borrow().left, &node2.as_ref().borrow().left)
                && is_same_tree(&node1.as_ref().borrow().right, &node2.as_ref().borrow().right);

        } else {
            return false
        }
    } else {
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree::tree_traverse::get_tree;

    #[test]
    fn test_is_same_tree() {
        let root1 = get_tree();
        let root2 = get_tree();
        assert!(is_same_tree(&root1, &root2));
    }
}