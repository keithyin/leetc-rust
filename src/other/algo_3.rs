use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_pos: HashMap<char, i32> = HashMap::new();
    let mut longest_length = 0;
    let mut sub_str_begin = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(mut pos) = char_pos.get_mut(&c) {
            sub_str_begin = *pos + 1;
            *pos = i as i32;
        } else {
            char_pos.insert(c, i as i32);
        }
        longest_length = if longest_length < (i as i32 - sub_str_begin) {
            i as i32 - sub_str_begin
        } else {
            longest_length
        };
    }
    longest_length as i32
}


#[cfg(test)]
mod test {
    use crate::other::algo_3::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        let value = String::from("abcca");
        println!("{}", length_of_longest_substring(value));
    }
}