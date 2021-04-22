use std::collections::HashMap;

pub fn num_trees(n: i32) -> i32{
    if n < 1 {
        return 0;
    }
    let mut cache = HashMap::new();
    return num_trees_core(1, n, &mut cache);
}

pub fn num_trees_core(begin: i32, end: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if begin >= end {
        return 1;
    }

    if let Some(val) = cache.get(&(end-begin)) {
        return *val;
    }

    let mut i = begin;
    let mut result = 0;
    while i <= end {
        let num_left = num_trees_core(begin, i-1, cache);
        let num_right = num_trees_core(i+1, end, cache);
        result += (num_left * num_right);
        i += 1;
    }
    cache.insert(end - begin, result);
    return result;
}