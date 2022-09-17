use std::cell::RefCell;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;


pub fn is_same_tree_core(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_some() {
        return false;
    }
    if p.is_some() && q.is_none() {
        return false;
    }

    if p.is_none() && q.is_none() {
        return true;
    }

    if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
        return false;
    }

    let left_is_same = is_same_tree_core(&p.as_ref().unwrap().borrow().left,
                                         &q.as_ref().unwrap().borrow().left);
    if ! left_is_same {
        return left_is_same;
    }
    let right_is_same = is_same_tree_core(&p.as_ref().unwrap().borrow().right,
                                          &q.as_ref().unwrap().borrow().right);
    return left_is_same && right_is_same;
}


pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {


    is_same_tree_core(&p, &q)
}