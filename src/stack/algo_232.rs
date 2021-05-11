use std::cell::RefCell;

#[derive(Debug)]
struct MyQueue {
    first_stack: RefCell<Vec<i32>>,
    second_stack: RefCell<Vec<i32>>,
    queue_order: RefCell<bool>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue{
            first_stack: RefCell::new(vec![]),
            second_stack: RefCell::new(vec![]),
            queue_order: RefCell::new(false),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&self, x: i32) {
        if *self.queue_order.borrow() {
            self.flip();
        }
        if !self.first_stack.borrow().is_empty() {
            self.first_stack.borrow_mut().push(x);
        } else {
            self.second_stack.borrow_mut().push(x);
        }
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&self) -> i32 {
        if !*self.queue_order.borrow() {
            self.flip();
        }

        if let Some(value) = self.first_stack.borrow_mut().pop() {
            return value;
        } else if let Some(value) = self.second_stack.borrow_mut().pop() {
            return value;
        }

        i32::MAX
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        if !*self.queue_order.borrow() {
            self.flip();
        }

        if !self.first_stack.borrow().is_empty() {
            return self.first_stack.borrow()[self.first_stack.borrow().len()-1];
        } else if self.second_stack.borrow().is_empty() {
            return self.second_stack.borrow()[self.second_stack.borrow().len()-1];
        }

        i32::MAX
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.first_stack.borrow().is_empty() && self.second_stack.borrow().is_empty()
    }

    fn flip(&self) {
        if !self.first_stack.borrow().is_empty() {
            while let Some(value)  = self.first_stack.borrow_mut().pop() {
                    self.second_stack.borrow_mut().push(value);
            }
        } else {
            while let Some(value)  = self.second_stack.borrow_mut().pop() {
                self.first_stack.borrow_mut().push(value);
            }
        }
        let queue_order = *self.queue_order.borrow();
        *self.queue_order.borrow_mut() = !queue_order;
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use crate::stack::algo_232::MyQueue;

    #[test]
    fn test_my_queue() {
        let queue = MyQueue::new();
        queue.push(1);
        queue.push(2);

        println!("{:?}", queue);
        let peek_val = queue.peek();
        println!("{:?}", queue);
        assert_eq!(peek_val, 1);
        let popped_val = queue.pop();
        println!("{:?}", queue);
        assert_eq!(popped_val, 1);

        assert!(!queue.empty());

    }
}