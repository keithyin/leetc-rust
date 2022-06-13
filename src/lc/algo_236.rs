use std::cell::RefCell;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

    let mut ca = None;
    lowest_common_ancestor_core(root, p, q, &mut ca);
    ca

}

fn lowest_common_ancestor_core(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>, ca: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
    


    if let Some(ref node) = root {
        if node.eq(&p.unwrap()) || node.eq(&q.unwrap()) {
            return true;
        }
        if let Some(ref left_node) = node.as_ref().borrow().left {
            left_found = lowest_common_ancestor_core(
                Some(Rc::clone(left_node)),
                Some(Rc::clone(&p.unwrap())),
            Some(Rc::clone(&q.unwrap())), ca);
        }

        if let Some(ref right_node) = node.as_ref().borrow().right{
            right_found = lowest_common_ancestor_core(
                Some(Rc::clone(right_node)),
                Some(Rc::clone(&p.unwrap())),
                Some(Rc::clone(&q.unwrap())), ca);
        }
        if right_found && left_found {
            *ca = Some(Rc::clone(node));
        }
        return right_found || left_found;

    }

    false
}