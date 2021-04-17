
pub struct MinStack{
    ori_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {

    pub fn new() -> Self {
        MinStack{ori_stack: Vec::new(), min_stack: Vec::new()}
    }

    pub fn push(&mut self, x: i32) {
        self.ori_stack.push(x);
        if self.min_stack.len() > 0 {
            if let Some(min_val) = self.min_stack.get(self.min_stack.len() - 1) {
                if x < *min_val {
                    self.min_stack.push(x);
                }
            }
        } else {
            self.min_stack.push(x);
        }

    }
    pub fn pop(&mut self) -> Option<i32> {
        if self.ori_stack.len() > 0 {
            let popped = self.ori_stack.pop();
            if self.ori_stack.len() > 0 {
                let mut pop_min_val = false;
                if let (Some(ref popped), Some(top_after_pop), Some(min_val)) = (popped, self.top(), self.get_min()) {
                    if *min_val == *popped && *popped != *top_after_pop {
                        pop_min_val = true;
                    }
                }
                if pop_min_val {
                    self.min_stack.pop();
                }
            } else {
                self.min_stack.pop();
            }

            return popped;
        }
        None
    }

    pub fn top(&self) -> Option<&i32> {
        if self.ori_stack.len() > 0 {
            self.ori_stack.get(self.ori_stack.len() - 1)
        } else {
            None
        }
    }

    pub fn get_min(&self) -> Option<&i32> {
        if self.min_stack.len() > 0 {
            self.min_stack.get(self.min_stack.len() - 1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(4);
        min_stack.push(2);
        min_stack.push(2);
        min_stack.push(3);
        min_stack.push(1);
        assert_eq!(min_stack.get_min(), Some(&1));
        assert_eq!(min_stack.pop(), Some(1));
        assert_eq!(min_stack.get_min(), Some(&2));
        assert_eq!(min_stack.pop(), Some(3));
        assert_eq!(min_stack.get_min(), Some(&2));
        assert_eq!(min_stack.pop(), Some(2));
        assert_eq!(min_stack.get_min(), Some(&2));

    }
}