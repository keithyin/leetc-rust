
struct MinStack {
    stack: Vec<i32>,
    minVal: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack{
            stack: vec![],
            minVal: vec![]
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.minVal.is_empty() {
            self.minVal.push(x);
        } else {
            if x <= self.minVal[self.minVal.len()-1] {
                self.minVal.push(x);
            }
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if val == self.minVal[self.minVal.len()-1] {
                self.minVal.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        return self.stack[self.stack.len()-1];
    }

    fn min(&self) -> i32 {
        return self.minVal[self.minVal.len()-1];
    }
}