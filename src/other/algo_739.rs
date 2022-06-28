
pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32>{
    let mut ans = vec![-1; temps.len()];
    let mut stack: Vec<usize> = vec![];
    for (i, v) in temps.iter().enumerate().rev() {
        while stack.len() > 0 && temps[stack[stack.len()-1]] < *v{
            stack.pop();
        }
        ans[i] = if stack.len() == 0 {
            0
        } else {
            (stack[stack.len()-1] - i) as i32
        };
        stack.push(i);
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::other::algo_739::daily_temperatures;

    #[test]
    fn test_rev() {
        let v = vec![1, 2, 3];
        for (i, v) in v.iter().enumerate().rev() {
            println!("{}:{}", i, v);
        }
    }

    #[test]
    fn test_daily_temps() {
        let items = vec![73, 74, 75, 71, 69, 72, 76, 73];
        println!("{:?}", daily_temperatures(items));
    }
}