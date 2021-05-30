use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;

pub fn target_path(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
    let tracer = Rc::new(RefCell::new(vec![]));
    let result = Rc::new(RefCell::new(vec![vec![]]));
    target_path_core(root, target, Rc::clone(&tracer), Rc::clone(&result));
    return result.as_ref().borrow().clone();
}

pub fn target_path_core(root: Option<Rc<RefCell<TreeNode>>>, mut target: i32, tracer:  Rc<RefCell<Vec<i32>>>, result:  Rc<RefCell<Vec<Vec<i32>>>>) {
    if let Some(node) = root {
        target -= node.as_ref().borrow().val;
        tracer.as_ref().borrow_mut().push(node.as_ref().borrow().val);
        if node.as_ref().borrow().right == None && node.as_ref().borrow().left == None {
            if target == 0 {
                result.as_ref().borrow_mut().push(tracer.as_ref().borrow().clone());
            }
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            target_path_core(
                Some(Rc::clone(right_node)), target, Rc::clone(&tracer), Rc::clone(&result));
        }

        if let Some(ref left_node) = node.as_ref().borrow().left {
            target_path_core(Some(Rc::clone(left_node)), target, Rc::clone(&tracer), Rc::clone(&result));
        }
        tracer.as_ref().borrow_mut().pop();
    }
}