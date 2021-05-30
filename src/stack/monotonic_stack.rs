
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
        let mut i= (ori_stack.len() - 1) as i32;
        while i >= 0 {
            while stack.len() > 0 &&  ori_stack[i as usize] > stack[stack.len()-1]{
                stack.pop();
            }
            monotonic_stack[i as usize] = if stack.len() > 0 {
                stack[stack.len()-1]
            } else {
                -1
            };
            stack.push(ori_stack[i as usize]);
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

#[derive(Debug)]
struct MonotonicStackV2 {
    results: Vec<usize>,
}

impl MonotonicStackV2 {
    pub fn new(ori_values: & Vec<i32>) -> Self {
        assert!(ori_values.len() > 0);
        let mut cursor = (ori_values.len() - 1) as i32;
        let mut stack = vec![];
        let mut results = vec![];
        results.resize(ori_values.len(), 0 as usize);
        while cursor >= 0 {
            let cur_value = ori_values[cursor as usize];
            while !stack.is_empty() && cur_value > ori_values[stack[stack.len()-1]] {
                stack.pop();
            }
            results[cursor as usize] = if stack.is_empty() {
                cursor as usize
            } else {
                stack[stack.len()-1]
            };
            stack.push(cursor as usize);
            cursor -= 1;
        }
        return MonotonicStackV2{
            results,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stack::monotonic_stack::{MonotonicStack, MonotonicStackV2};

    #[test]
    fn test_monotonic_stack() {
        let a = vec![1, 2, 3, 2, 2];
        let monotonic_stack = MonotonicStack::new(&a);
        println!("{:?}", monotonic_stack);
    }

    #[test]
    fn test_monotonic_stack_v2() {
        let a = vec![1, 2, 3, 4, 5];
        println!("{:?}", MonotonicStackV2::new(&a));
    }
}