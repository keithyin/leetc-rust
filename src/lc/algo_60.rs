use std::collections::HashSet;

pub fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut res = 1;
    for i in 2..n+1 {
        res *= i;
    }
    res
}
