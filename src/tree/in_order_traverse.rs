// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut trace = vec![];
    inorder_traversal_core(&root, &mut trace);
    return trace;
}

fn inorder_traversal_core(root: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
    if let Some(ref node) = root {
        if let Some(ref node) = node.as_ref().borrow().left {
            inorder_traversal_core(&Some(Rc::clone(node)), trace);
        } else {
            inorder_traversal_core(&None, trace);
        }

        trace.push(node.as_ref().borrow().val);

        if let Some(ref node) = node.as_ref().borrow().right {
            inorder_traversal_core(&Some(Rc::clone(node)), trace);
        } else {
            inorder_traversal_core(&None, trace);
        }
    }
}

fn inorder_traverse_stack(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    while let Some(ref node) = root {
        if let Some(ref node) = node.as_ref().borrow().left {
            stack.push(Rc::clone(node));
        } else {
            
        }
    }
    return vec![1, 2];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rc() {
        let mut a = Rc::new(RefCell::new(3));
        let mut b = a.clone();
        println!("{:?}, {:?}", a, b);
        *(a.borrow_mut()) = 10;
        println!("{:?}, {:?}", a, b);
    }

    fn get_tree() -> Option<Rc<RefCell<TreeNode>>>{
        let mut tree = TreeNode::new(10);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(32))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(11))));
        Some(Rc::new(RefCell::new(tree)))
    }

    #[test]
    fn test_inorder_traverse() {
        let tree = get_tree();
        let inorder_res = inorder_traversal(tree);
        assert_eq!(inorder_res, vec![32, 10, 11]);
    }
}