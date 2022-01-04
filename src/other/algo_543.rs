/*
    因为是树相关的问题，所以上来就开始考虑如何使用4种遍历方式来解决了。
    直径一定由某个root开始的左边长度 + 右边长度。想到这个就可以使用前序遍历的方式来解决了。
    每当遍历一个节点
*/

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

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use std::env::var;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let mut diameter_list = vec![];
  if let Some(ref root) = root {
    core(root, &mut diameter_list);
  }
  if let Some(val) = diameter_list.iter().max() {
    *val
  } else {
    0
  }
}


pub fn core(root: &Rc<RefCell<TreeNode>>, diameter_list: &mut Vec<i32>) -> (i32) {
  if root.as_ref().borrow().left == None && root.as_ref().borrow().right == None {
    return 0;
  }
  let mut left_single_side_max_len = -1;
  if let Some(ref left_node) = root.as_ref().borrow().left {
     left_single_side_max_len = core(left_node, diameter_list);
  }

  let mut right_single_side_max_len = -1;
  if let Some(ref right_node) = root.as_ref().borrow().right {
    right_single_side_max_len = core(right_node, diameter_list);
  }

  diameter_list.push(2+left_single_side_max_len + right_single_side_max_len);

  max(1+left_single_side_max_len, 1+right_single_side_max_len)
}