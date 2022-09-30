use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use crate::lc::algo_297::TreeNode;

struct BSTIterator {
    stack: LinkedList<Rc<RefCell<TreeNode>>>,
    cur_head: Option<Rc<RefCell<TreeNode>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = LinkedList::new();
        if let Some(ref node) = root {
            stack.push_back(Rc::clone(node));
        }
        let cur_head = if let Some(ref node) = root {
            Some(Rc::clone(node))
        } else {
            None
        };
        BSTIterator{stack, cur_head}
    }

    fn next(&mut self) -> i32 {
        loop {
            if self.cur_head.is_none() {
                let cur_val = self.stack.pop_back().unwrap();
                self.cur_head = if let Some(ref node) = cur_val.as_ref().borrow().right {
                    Some(Rc::clone(node))
                } else {
                    None
                };

                return cur_val.as_ref().borrow().val;
            } else {
                println!("{}", self.cur_head.as_ref().unwrap().as_ref().borrow().val);
                self.stack.push_back(Rc::clone(self.cur_head.as_ref().unwrap()));
                let new_head = if let Some(ref node) = self.cur_head.as_ref().unwrap().as_ref().borrow().left {
                    Some(Rc::clone(node))
                } else {
                    None
                };
                self.cur_head = new_head;

            }

        }
    }

    fn has_next(&self) -> bool {
        println!("{:?}", self.stack.len());
        self.stack.len() > 0
    }
}

#[cfg(test)]
mod test{
    #[test]
    pub fn test(){

    }
}