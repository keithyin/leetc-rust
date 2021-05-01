use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;


pub fn get_tree1() -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree = TreeNode::new(10);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(32))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(11))));
    if let Some(ref node) = tree.left {
        node.as_ref().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    }
    Some(Rc::new(RefCell::new(tree)))
}

pub fn get_tree2() -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree = TreeNode::new(4);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(32))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(11))));
    if let Some(ref node) = tree.left {
        node.as_ref().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    }
    Some(Rc::new(RefCell::new(tree)))
}

pub fn get_symmetric_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree = TreeNode::new(4);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(32))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(32))));
    Some(Rc::new(RefCell::new(tree)))
}