pub mod min_stack;
mod min_array;
mod word_search;
mod moving_count;


pub mod algo {
    use std::cell::RefCell;

    pub struct CQueue {
        vec1: RefCell<Vec<i32>>,
        vec2: RefCell<Vec<i32>>
    }


    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl CQueue {

        pub fn new() -> Self {
            CQueue{
                vec1: RefCell::new(vec![]),
                vec2: RefCell::new(vec![]),
            }
        }

        pub fn append_tail(&self, value: i32) {
            if self.vec1.borrow().len() > 0 {
                self.vec1.borrow_mut().push(value);
            }else {
                self.vec2.borrow_mut().push(value);
            }
        }
        pub fn delete_head(&self) -> i32 {
            if self.vec1.borrow().len()>0 {
                while let Some(val) = self.vec1.borrow_mut().pop() {
                    self.vec2.borrow_mut().push(val)
                }
            } else {
                while let Some(val) = self.vec2.borrow_mut().pop() {
                    self.vec1.borrow_mut().push(val);
                }
            }
            let res = if self.vec1.borrow().len() > 0 {
                if let Some(val) = self.vec1.borrow_mut().pop() {
                    val
                } else {
                    -1
                }

            } else {
                if let Some(val) = self.vec2.borrow_mut().pop() {
                    val
                } else {
                    -1
                }
            };
            if self.vec1.borrow().len()>0 {
                while let Some(val) = self.vec1.borrow_mut().pop() {
                    self.vec2.borrow_mut().push(val)
                }
            } else {
                while let Some(val) = self.vec2.borrow_mut().pop() {
                    self.vec1.borrow_mut().push(val);
                }
            }
            res
        }
    }

}

#[cfg(test)]
mod test {

    #[test]
    fn test_cqueue() {
        let cq = super::algo::CQueue::new();
        cq.append_tail(1);
        cq.append_tail(2);
        println!("{:?}", cq.delete_head());
        cq.append_tail(3);
        println!("{:?}", cq.delete_head());

    }

    #[test]
    fn test_fib() {
        pub fn fib(n: i32) -> i32 {
            if n == 0 {
                return 0;
            }
            if n == 1 {
                return 1;
            }
            let mut n_1: u64 = 1;
            let mut n_2: u64 = 0;
            for _ in 2..(n+1) {
                let tmp_n_1 = n_1;
                n_1 = n_1 + n_2;
                n_2 = tmp_n_1;
            }
            (n_1 % 1000000007) as i32
        }
    }
}