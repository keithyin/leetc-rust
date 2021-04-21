
pub fn num_trees(n: i32) -> i32{
    if n < 1 {
        return 0;
    }
    return num_trees_core(1, n);
}

pub fn num_trees_core(begin: i32, end: i32) -> i32 {
    if begin > end {
        return 1;
    }

    let mut i = begin;
    let mut result = 0;
    while i <= end {
        let num_left = num_trees_core(begin, i-1);
        let num_right = num_trees_core(i+1, end);
        result += (num_left * num_right);
        i += 1;
    }
    return result;
}