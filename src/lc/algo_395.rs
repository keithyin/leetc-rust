use std::collections::HashMap;

// 核心就是利用，若果一个 字符，全局出现不超过 k，那么就可以根据该字符将字符串切分
pub fn longest_substring(s: String, k: i32) -> i32 {
    let mut counter: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if counter.contains_key(&c) {
            counter.insert(c, counter[&c] + 1);
        } else {
            counter.insert(c, 1);
        }
    }

    let mut min_count_c = ' ';
    let mut min_count = i32::MAX;
    for (c, count) in &counter {
        if min_count > *count {
            min_count = *count;
            min_count_c = *c;
        }
    }
    if min_count >= k {
        return s.len() as i32;
    }
    let splitted_strs: Vec<&str> = s.split(min_count_c).collect();
    let mut length = 0;
    for sub_str in splitted_strs {
        let sub_sub_str_len = longest_substring(String::from(sub_str), k);
        length = if length < sub_sub_str_len {
            sub_sub_str_len
        } else {
            length
        };
    }

    length
}

#[cfg(test)]
mod test {
    use crate::lc::algo_395::longest_substring;

    #[test]
    fn test_longest_substring() {
        let str = String::from("aabbccc");

        println!("{}", longest_substring(str, 2))

    }
}