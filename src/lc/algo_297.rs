
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

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
            right: None
        }
    }
}

struct Codec {

}

/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead
  *
*/
impl Codec {
    fn new() -> Self {
        Codec{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut queue = vec![];
        let mut result = vec![];
        if let Some(node) = root {
            queue.push(Some(Rc::clone(&node)));
        }
        while queue.len() > 0 {
            let mut node = queue.remove(0);
            if let Some(inner_node) = node {
                result.push(inner_node.as_ref().borrow().val.to_string());

                if let Some(ref left_node) = inner_node.as_ref().borrow().left {
                    queue.push(Some(Rc::clone(left_node)));
                } else {
                    queue.push(None);
                }

                if let Some(ref right_node) = inner_node.as_ref().borrow().right {
                    queue.push(Some(Rc::clone(right_node)));
                } else {
                    queue.push(None);
                }

            } else {
                result.push("null".to_string());
            }

        }
        result.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = vec![];
        let mut head = None;

        let strs :Vec<&str> = data.split(",").collect();
        let mut index = 0;
        loop {
            if index >= strs.len() {
                break;
            }
            if index == 0 {
                if strs[index] != "null" {
                    let tmp_h = Rc::new(
                        RefCell::new(TreeNode::new(
                            i32::from_str(strs[index]).unwrap())));
                    queue.push(Rc::clone(&tmp_h));

                } else {
                    return None;
                }
                index += 1;
                continue;
            }

            let node = queue.remove(0);

            let left_str = &strs[index];
            if left_str != &"null" {
                let val = i32::from_str(left_str).unwrap();
                let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                queue.push(Rc::clone(&left_node));
                node.borrow_mut().left = Some(Rc::clone(&left_node));
            }
            index += 1;
            let right_str = &strs[index];
            if right_str != &"null" {
                let val = i32::from_str(right_str).unwrap();
                let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                queue.push(Rc::clone(&right_node));
                node.borrow_mut().left = Some(Rc::clone(&right_node));
            }

            index += 1;
        }



        head
    }
}
