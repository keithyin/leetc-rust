use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn path_sum_core(root: &Option<Rc<RefCell<TreeNode>>>, mut residual: i32, tracer: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if root.is_none() {
        return;
    }

    residual = residual - root.as_ref().unwrap().as_ref().borrow().val;
    tracer.push(root.as_ref().unwrap().as_ref().borrow().val);
    if residual == 0 && root.as_ref().unwrap().as_ref().borrow().left.is_none() && root.as_ref().unwrap().as_ref().borrow().right.is_none() {
        result.push(tracer.clone());
        return;
    }

    path_sum_core(&root.as_ref().unwrap().as_ref().borrow().left, residual, tracer, result);
    path_sum_core(&root.as_ref().unwrap().as_ref().borrow().right, residual, tracer, result);

    tracer.pop();
}


pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut tracer = vec![];
    path_sum_core(&root, target_sum, &mut tracer, &mut result);
    result
}