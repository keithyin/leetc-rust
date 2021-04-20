
use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::tree_traverse::TreeNode;


pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    return vec![];
}

pub fn generate_trees_core(begin: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if begin < end {
        return vec![None];
    }
    let mut i = begin;
    while i < (end+1) {
        let cur_root = Some(Rc::new(RefCell::new(TreeNode::new(i))));
    }
    return vec![];
}