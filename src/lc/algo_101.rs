use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::lc::algo_297::TreeNode;

pub fn is_symmetric_core(root1: Option<Rc<RefCell<TreeNode>>>,
                         root2: Option<Rc<RefCell<TreeNode>>>) -> bool {

    if root1.is_none() && root2.is_none() {
        return true;
    }
    if root1.is_none() {
        return false;
    }
    if root2.is_none() {
        return false;
    }
    if let (Some(root1_inner), Some(root2_inner)) = (&root1, &root2) {
        if root1_inner.as_ref().borrow().val != root2_inner.as_ref().borrow().val {
            return false;
        }

        let root1_left = if let Some(inner) = &root1_inner.as_ref().borrow().left {
            Some(Rc::clone(inner))
        } else {
            None
        };

        let root1_right = if let Some(inner) = &root1_inner.as_ref().borrow().right {
            Some(Rc::clone(inner))
        } else {
            None
        };

        let root2_left = if let Some(inner) = &root2_inner.as_ref().borrow().left {
            Some(Rc::clone(inner))
        } else {
            None
        };

        let root2_right = if let Some(inner) = &root2_inner.as_ref().borrow().right {
            Some(Rc::clone(inner))
        } else {
            None
        };

        return is_symmetric_core(root1_left, root2_right) && is_symmetric_core(root1_right, root2_left);

    }
    true
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root1 = if let Some(inner) = &root {
        Some(Rc::clone(inner))
    } else {
        None
    };
    let root2 = if let Some(inner) = &root {
        Some(Rc::clone(inner))
    } else {
        None
    };
    return is_symmetric_core(root1, root2);
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_is_symmetric() {

    }
}