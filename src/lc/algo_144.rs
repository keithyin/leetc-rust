use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = LinkedList::new();
    if let Some(ref node) = root {
        stack.push_back(Rc::clone(node));
    } else {
        return vec![];
    }

    let mut result = vec![];
    while stack.len() > 0 {
        let cur_node = stack.pop_back().unwrap();
        result.push(cur_node.as_ref().borrow().val);

        if let Some(right_node) = cur_node.as_ref().borrow().right.as_ref() {
            stack.push_back(Rc::clone(right_node));
        }
        if let Some(left_node) = cur_node.as_ref().borrow().left.as_ref() {
            stack.push_back(Rc::clone(left_node));
        };
    }

    result
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {

    }
}