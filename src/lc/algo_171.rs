
pub fn title_to_number(column_title: String) -> i32 {
    let mut base = 1;
    let mut result = 0;
    for c in column_title.chars().rev() {
        result += (c as u32 - 'A' as u32 + 1) * base;
        base *= 26;
    }

    result as i32
}

#[cfg(test)]
mod test {
    use crate::lc::algo_171::title_to_number;

    #[test]
    fn test_is_palindrome() {
        println!("{}", title_to_number("ZY".to_string()));
    }
}