
/*
abcabcbb

 */
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut max_len = 0;
    let mut begin = 0;
    let mut map = HashMap::new();
    for i in 0..s.len() {
        let cur_c = s[i];
        if map.contains_key(&cur_c) && map[&cur_c] >= begin{
            let cur_len = i - begin;
            max_len = if max_len < cur_len {cur_len} else {max_len};
            begin = map[&cur_c] + 1;
        }
        map.insert(cur_c, i);
    }
    let cur_len = s.len() - begin;
    max_len = if max_len < cur_len {cur_len} else {max_len};


    max_len as i32
}