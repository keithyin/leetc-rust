use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;

pub fn list_2_number(vals: &Vec<i32>) -> i32 {
    if vals.len() == 0 {
        return 0;
    }
    let mut result = 0;
    let mut base = 1;
    let mut cursor = (vals.len()-1) as i32;
    while cursor >= 0 {
        result += vals[cursor as usize] * base;
        base *= 10;
        cursor -= 1;
    }
    result
}

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum_val = 0;
    let tracer = Rc::new(RefCell::new(vec![]));
    let result = Rc::new(RefCell::new(vec![]));
    sum_numbers_core(root, Rc::clone(&tracer), Rc::clone(&result));
    for vals in result.as_ref().borrow().iter() {
        sum_val += list_2_number(vals);
    }
    sum_val
}

pub fn sum_numbers_core(root: Option<Rc<RefCell<TreeNode>>>, tracer: Rc<RefCell<Vec<i32>>>,
                        result: Rc<RefCell<Vec<Vec<i32>>>>) {
    if let Some(node) = root {
        tracer.as_ref().borrow_mut().push(node.as_ref().borrow().val);
        if node.as_ref().borrow().left == None && node.as_ref().borrow().right == None {
            result.as_ref().borrow_mut().push(tracer.as_ref().borrow().clone());
        }
        if let Some(ref left_node) = node.as_ref().borrow().left {
            sum_numbers_core(Some(Rc::clone(left_node)), Rc::clone(&tracer),
            Rc::clone(&result));
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            sum_numbers_core(Some(Rc::clone(right_node)), Rc::clone(&tracer),
                             Rc::clone(&result));
        }
        tracer.as_ref().borrow_mut().pop();
    }
}

#[cfg(test)]
mod test {
    use crate::tree::algo_129::{list_2_number, sum_numbers};
    use crate::tree::build_tree::{get_tree_for_sum, get_tree_for_sum_2};

    #[test]
    fn test_list_2_number() {
        let vals = vec![1, 2, 3];
        assert_eq!(list_2_number(&vals), 123);
    }

    #[test]
    fn test_sum_numbers() {
        let tree = get_tree_for_sum();
        assert_eq!(sum_numbers(tree), 419);
        let tree = get_tree_for_sum_2();
        assert_eq!(sum_numbers(tree), 455);
    }
}