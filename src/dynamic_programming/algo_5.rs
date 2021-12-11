
pub fn longest_palindrome(s: String) -> String {
    let string_list: Vec<char> = s.chars().into_iter().collect();
    let tmp_string: String = string_list.join('#');

    let mut longest_begin = 0;
    let mut longest_end = 0;
    let mut longest = 0;
    let mut begin = 0;
    let mut end = 0;


}