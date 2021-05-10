use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;
use std::rc::Rc;
use std::collections::VecDeque;

pub fn zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut first_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut second_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut result = vec![];
    if let Some(ref node) = root {
        first_queue.push_back(Rc::clone(node));
    }
    while !first_queue.is_empty() || !second_queue.is_empty() {
        let mut local_vec: Vec<i32> = vec![];
        while !first_queue.is_empty() {
            if let Some(node) = first_queue.pop_front() {
                local_vec.push(node.as_ref().borrow().val);
                if let Some(ref node) = node.as_ref().borrow().left {
                    second_queue.push_back(Rc::clone(node));
                }
                if let Some(ref node) = node.as_ref().borrow().right {
                    second_queue.push_back(Rc::clone(node));
                }
            }
        }
        if !local_vec.is_empty() {
            result.push(local_vec);
        }
        let mut local_vec: Vec<i32> = vec![];
        while !second_queue.is_empty() {
            if let Some(node) = second_queue.pop_front() {
                local_vec.insert(0,node.as_ref().borrow().val);
                if let Some(ref node) = node.as_ref().borrow().left {
                    first_queue.push_back(Rc::clone(node));
                }
                if let Some(ref node) = node.as_ref().borrow().right {
                    first_queue.push_back(Rc::clone(node));
                }
            }
        }
        if !local_vec.is_empty() {
            result.push(local_vec);
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use crate::tree::tree_traverse::get_tree;
    use crate::tree::algo_103::zig_zag;

    #[test]
    fn test_level_order() {
        let tree = get_tree();
        let result = zig_zag(tree);
        assert_eq!(result, vec![vec![10], vec![11, 32], vec![9]]);
    }
}