use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_traverse::TreeNode;

pub fn common_ancestor(root: Option<Rc<RefCell<TreeNode>>>,
                       node1: i32,
                       node2: i32, result: &mut Option<Rc<RefCell<TreeNode>>>) -> (bool) {
    if let Some(node) = root {
        if node.as_ref().borrow().val == node1 || node.as_ref().borrow().val == node2 {
            return true;
        }
        let mut state = 0;
        if let Some(ref left_node) = node.as_ref().borrow().left {
            let left_hit = common_ancestor(Some(Rc::clone(left_node)), node1, node2, result);
            if left_hit {
                state += 1;
            }
        }
        if let Some(ref right_node) = node.as_ref().borrow().right {
            let right_hit = common_ancestor(Some(Rc::clone(right_node)), node1, node2, result);
            if right_hit {
                state += 1;
            }
        }

        if state == 2 {
            *result = Some(node.clone())
        }
        return state > 0;
    }
    false
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::tree::tree_traverse::get_tree;
    use crate::tree::algo_236::common_ancestor;

    fn consume_ref(a: &mut i32) {
        *a = 10;
        println!("{}", *a);
    }

    #[test]
    fn test_rc_equal() {
        let rc1 = Rc::new(2);
        let rc2 = Rc::clone(&rc1);
        assert!(rc1 == rc2);
        assert!(&rc1 == &rc2);
        assert!(Some(rc1) == Some(rc2));
        let mut a = 0;
        let mut b = 1;
        let ref_a = &mut a;
        consume_ref(ref_a);
        consume_ref(ref_a);  // TODO: why it works, 可以发现，传&mut的方法，都可以将 &mut返回出来。这样就使得这种方式非常合理了？
        println!("{}", ref_a);
        // let ref_a_2 = ref_a;
        // println!("{}", ref_a);  // this doesn't work make sense.
    }

    #[test]
    fn test_common_ancestor() {
        let tree = get_tree();
        let mut result = None;
        let hit = common_ancestor(tree, 32, 11, &mut result);
        println!("{:?}", result);
        println!("{}", hit);

    }
}