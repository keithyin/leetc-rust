use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
struct MyStack {
    first_queue: RefCell<VecDeque<i32>>,
    second_queue: RefCell<VecDeque<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack{
            first_queue: RefCell::new(VecDeque::new()),
            second_queue: RefCell::new(VecDeque::new()),
        }
    }

    /** Push element x onto stack. */
    fn push(&self, x: i32) {
        if !self.first_queue.borrow().is_empty() {
            self.first_queue.borrow_mut().push_back(x);
        }else {
            self.second_queue.borrow_mut().push_back(x);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&self) -> i32 {
        if !self.first_queue.borrow().is_empty() {
            while self.first_queue.borrow().len() > 1 {
                if let Some(value) = self.first_queue.borrow_mut().pop_front() {
                    self.second_queue.borrow_mut().push_back(value);
                }
            }
            if let Some(value) = self.first_queue.borrow_mut().pop_front() {
                return value
            }
        }else if !self.second_queue.borrow().is_empty() {
            while self.second_queue.borrow().len() > 1 {
                if let Some(value) = self.second_queue.borrow_mut().pop_front() {
                    self.first_queue.borrow_mut().push_back(value)
                }
            }
            if let Some(value) = self.second_queue.borrow_mut().pop_front() {
                return value;
            }
        }

        i32::MAX
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        if !self.first_queue.borrow().is_empty() {
            while self.first_queue.borrow().len() > 1 {
                if let Some(value) = self.first_queue.borrow_mut().pop_front() {
                    self.second_queue.borrow_mut().push_back(value);
                }
            }
            if let Some(value) = self.first_queue.borrow_mut().pop_front() {
                self.second_queue.borrow_mut().push_back(value);
                return value
            }
        }else if !self.second_queue.borrow().is_empty() {
            while self.second_queue.borrow().len() > 1 {
                if let Some(value) = self.second_queue.borrow_mut().pop_front() {
                    self.first_queue.borrow_mut().push_back(value)
                }
            }
            if let Some(value) = self.second_queue.borrow_mut().pop_front() {
                self.first_queue.borrow_mut().push_back(value);
                return value;
            }
        }

        i32::MAX
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.first_queue.borrow().is_empty() && self.second_queue.borrow().is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use crate::stack::algo_225::MyStack;

    #[test]
    fn test_mystack() {
        let stack = MyStack::new();
        stack.push(1);
        stack.push(2);

        let top_res = stack.top();
        assert_eq!(top_res, 2);
        let popped_res = stack.pop();
        assert_eq!(popped_res, 2);
        let is_empty = stack.empty();
        assert!(!is_empty);
    }
}