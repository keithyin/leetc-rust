
use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::tree_traverse::TreeNode;


pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n < 1 {
        return vec![];
    }
    return generate_trees_core(1, n);
}

pub fn generate_trees_core(begin: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if begin > end {
        return vec![None];
    }
    let mut res: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    let mut i = begin;
    while i <= end {
        let left_trees = generate_trees_core(begin, i-1);
        let right_trees = generate_trees_core(i+1, end);
        for left_node in left_trees.iter(){
            for right_node in right_trees.iter() {
                let cur_node = Rc::new(RefCell::new(TreeNode::new(i)));
                if let Some(left_node) = left_node {
                    cur_node.as_ref().borrow_mut().left = Some(Rc::clone(left_node));
                } else {
                    cur_node.as_ref().borrow_mut().left = None;
                }
                if let Some(right_node) = right_node {
                    cur_node.as_ref().borrow_mut().right = Some(Rc::clone(right_node));
                } else {
                    cur_node.as_ref().borrow_mut().right = None;
                }
                res.push(Some(cur_node));
            }
        }
        i += 1;
    }
    return res;
}