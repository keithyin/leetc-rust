
#[derive(Debug)]
struct MonotonicStack<'a> {
    ori_stack: &'a Vec<i32>,
    monotonic_stack: Vec<i32>,
}

impl <'a> MonotonicStack<'a> {
    pub fn new(ori_stack: &'a Vec<i32>) -> Self {
        let mut monotonic_stack = vec![];
        let mut i = 0;
        while i < ori_stack.len() {
            monotonic_stack.push(0);
            i += 1;
        }

        let mut stack = vec![];
        let mut i= ori_stack.len() - 1;
        while i >= 0 {
            while stack.len() > 0 &&  ori_stack[i] > stack[stack.len()-1]{
                stack.pop();
            }
            monotonic_stack[i] = if stack.len() > 0 {
                stack[stack.len()-1]
            } else {
                -1
            };
            stack.push(ori_stack[i]);
            if i == 0 {
                break;
            }
            i -= 1;
        }


        MonotonicStack{
            ori_stack,
            monotonic_stack,
        }

    }
}

#[cfg(test)]
mod test {
    use crate::stack::monotonic_stack::MonotonicStack;

    #[test]
    fn test_monotonic_stack() {
        let a = vec![1, 2, 3, 2, 2];
        let monotic_stack = MonotonicStack::new(&a);
        println!("{:?}", monotic_stack);
    }
}