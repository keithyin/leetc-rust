// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

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
    return vec![1, 2];
}

// fn inorder_traversal_core(root: Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
//     if let Some(ref node) = root {
//         inorder_traversal_core(node.borrow().left, trace);
//         trace.insert(node.borrow().val);
//         inorder_traversal_core(node.borrow().right, trace);
//     }
// }

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
}