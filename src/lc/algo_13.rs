use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let map = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000)
    ]);
    let s = s.as_str();

    let mut pre_v = 0;
    let mut result = 0;
    for i in 0..s.len() {
        let cur_v = map[&s[i..i+1]];
        if pre_v > 0 && pre_v < cur_v {
            result = result - 2 * pre_v + cur_v;
        } else {
            result += cur_v;
        }
        pre_v = cur_v;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_13::roman_to_int;

    #[test]
    pub fn test() {
        println!("{}", roman_to_int("IV".to_string()));
    }
}