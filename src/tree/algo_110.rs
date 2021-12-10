use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;
use std::rc::Rc;

pub fn is_balance_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool{
    let (is_balance, _) = is_balance_tree_core(root);
    is_balance
}

pub fn is_balance_tree_core(root: Option<Rc<RefCell<TreeNode>>>)->(bool, i32) {
    if let Some(node) = root {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(ref left_node) = node.as_ref().borrow().left {
             let (left_balance, left_height_inner) = is_balance_tree_core(Some(Rc::clone(left_node)));
            if !left_balance {
                return (false, 0);
            }
            left_height = left_height_inner;
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            let (right_balance, right_height_inner) = is_balance_tree_core(Some(Rc::clone(right_node)));
            if !right_balance {
                return (false, 0);
            }
            right_height = right_height_inner;
        }
        let height_delta = left_height - right_height;
        if height_delta > 1 || height_delta < -1 {
            return (false, 0);
        }
        let max_height = if right_height > left_height {
            right_height
        }else {
            left_height
        };
        return (true, max_height + 1);
    }
    return (true, 0);

}

#[cfg(test)]
mod test {
    use crate::tree::tree_traverse::get_tree;
    use crate::tree::algo_110::is_balance_tree;
    use crate::tree::build_tree::get_non_balance_tree;

    #[test]
    fn test_is_balance_tree() {
        let tree = get_tree();
        println!("{}", is_balance_tree(tree));
        println!("{}", is_balance_tree(get_non_balance_tree()));
    }
}