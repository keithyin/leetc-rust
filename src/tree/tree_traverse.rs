// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option::Some;
use std::collections::VecDeque;

pub fn get_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree = TreeNode::new(10);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(32))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(11))));
    if let Some(ref node) = tree.left {
        node.as_ref().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    }
    Some(Rc::new(RefCell::new(tree)))
}

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
    let mut trace = vec![];
    inorder_traversal_core(&root, &mut trace);
    return trace;
}

fn inorder_traversal_core(root: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
    if let Some(ref node) = root {
        if let Some(ref node) = node.as_ref().borrow().left {
            inorder_traversal_core(&Some(Rc::clone(node)), trace);
        } else {
            inorder_traversal_core(&None, trace);
        }

        trace.push(node.as_ref().borrow().val);

        if let Some(ref node) = node.as_ref().borrow().right {
            inorder_traversal_core(&Some(Rc::clone(node)), trace);
        } else {
            inorder_traversal_core(&None, trace);
        }
    }
}

fn inorder_traverse_stack(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut res: Vec<i32> = Vec::new();
    if let Some(ref node) = root {
        stack.push(Rc::clone(node));
    }
    while stack.len() > 0 || root != None {
        while root != None {
            let mut next_root = None;
            if let Some(ref node) = root {
                if let Some(ref child) = node.as_ref().borrow().left {
                    next_root = Some(Rc::clone(child));
                    stack.push(Rc::clone(child));
                }
            }
            root = next_root;
        }

        if let Some(val) = stack.pop() {
            res.push(val.as_ref().borrow().val);
            if let Some(ref val) = val.as_ref().borrow().right {
                root = Some(Rc::clone(val));
                stack.push(Rc::clone(val));
            } else {
                root = None;
            }
        }

    }
    return res;
}


fn preorder_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut trace:Vec<i32> = Vec::new();
    preorder_traverse_core(&root, &mut trace);
    return trace;
}

fn preorder_traverse_core(root: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
    if let Some(node) = root {
        trace.push(node.as_ref().borrow().val);
        preorder_traverse_core(&node.as_ref().borrow().left, trace);
        preorder_traverse_core(&node.as_ref().borrow().right, trace);
    }
}

fn preorder_stack(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut result: Vec<i32> = Vec::new();
    while stack.len() > 0 || root != None {
        while root != None {
            let mut next_root = None;
            if let Some(ref node) = root {
                result.push(node.as_ref().borrow().val);
                stack.push(Rc::clone(node));
                if let Some(ref node) = node.as_ref().borrow().left {
                    next_root = Some(Rc::clone(node));
                } else {
                    next_root = None;
                }
            }
            root = next_root;
        }
        if let Some(node) = stack.pop() {
            if let Some(ref node) = node.as_ref().borrow().right {
                root = Some(Rc::clone(node));
            } else {
                root = None;
            }
        }
    }
    return result;
}

fn postorder_traverse(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut trace: Vec<i32> = Vec::new();
    postorder_traverse_core(root, &mut trace);
    return trace;
}

fn postorder_traverse_core(root: &Option<Rc<RefCell<TreeNode>>>, trace: &mut Vec<i32>) {
    if let Some(node) = root {
        postorder_traverse_core(&node.as_ref().borrow().left, trace);
        postorder_traverse_core(&node.as_ref().borrow().right, trace);
        trace.push(node.as_ref().borrow().val);
    }
}

fn postorder_traverse_stack(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut result: Vec<i32> = Vec::new();
    let mut prev = Rc::new(RefCell::new(TreeNode::new(0)));
    while stack.len()>0 || root != None {
        while root != None {
            let mut next_root: Option<Rc<RefCell<TreeNode>>> = None;
            if let Some(ref node) = root {
                stack.push(Rc::clone(node));
                if let Some(ref node) = node.as_ref().borrow().left {
                    next_root = Some(Rc::clone(node));
                }
            }
            root = next_root;

        }
        let top = Rc::clone(&stack[stack.len()-1]);
        if let Some(ref node) = top.as_ref().borrow().right {
            if *node != prev {
                root = Some(Rc::clone(node));
            } else {
                if let Some(top) = stack.pop() {
                    result.push(top.as_ref().borrow().val);
                    prev = Rc::clone(&top);
                    root = None;
                }
            }
        } else {
            if let Some(top) = stack.pop() {
                result.push(top.as_ref().borrow().val);
                prev = top;
                root = None;
            }
        };
    }
    return result;
}

pub fn layer_wise_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
    let mut deque = VecDeque::new();
    let mut results = vec![];
    if let Some(ref node) = root {
        deque.push_back(Rc::clone(node));
    }

    while let Some(node) = deque.pop_front(){
        results.push(node.as_ref().borrow().val);
        if let Some(ref left) = node.as_ref().borrow().left {
            deque.push_back(Rc::clone(left));
        }
        if let Some(ref right) = node.as_ref().borrow().right {
            deque.push_back(Rc::clone(right));
        }
    }
    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rc() {
        let a = Rc::new(RefCell::new(3));
        let b = a.clone();
        println!("{:?}, {:?}", a, b);
        *(a.as_ref().borrow_mut()) = 10;
        println!("{:?}, {:?}", a, b);

        let tree = get_tree();
        let mut stack: Vec<&Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        if let Some(ref node) = tree {
            stack.push(&node.as_ref().borrow().left);
        }
    }

    #[test]
    fn test_inorder_traverse() {
        let tree = get_tree();
        let inorder_res = inorder_traversal(tree);
        assert_eq!(inorder_res, vec![32, 9, 10, 11]);
    }

    #[test]
    fn test_inorder_traverse_stack() {
        let tree = get_tree();
        let inorder_res = inorder_traverse_stack(tree);
        assert_eq!(inorder_res, vec![32, 9, 10, 11]);
    }

    #[test]
    fn test_preorder_traverse() {
        let tree = get_tree();
        let preorder_res = preorder_traverse(tree);
        assert_eq!(preorder_res, vec![10, 32, 9, 11]);
    }

    #[test]
    fn test_preorder_traverse_stack() {
        let tree = get_tree();
        let preorder_res = preorder_stack(tree);
        assert_eq!(preorder_res, vec![10, 32, 9, 11]);
    }

    #[test]
    fn test_postorder_traverse() {
        let tree = get_tree();
        let postorder_res = postorder_traverse(&tree);
        assert_eq!(postorder_res, vec![9, 32, 11, 10]);
    }

    #[test]
    fn test_postorder_traverse_stack() {
        let tree = get_tree();
        let postorder_res = postorder_traverse_stack(tree);
        assert_eq!(postorder_res, vec![9, 32, 11, 10]);
    }

    #[test]
    fn test_ref_eq() {
        let a = Rc::new(5);
        let b = a.clone();
        let _c = Rc::clone(&b);
        assert_eq!(a.as_ref(), b.as_ref());
    }

    #[test]
    fn test_layer_wise_traverse() {
        let root = get_tree();
        let result = layer_wise_traverse(root);
        assert_eq!(result, vec![10, 32, 11, 9]);
    }
}