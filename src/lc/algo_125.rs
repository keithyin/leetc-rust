
pub fn is_palindrome(s: String) -> bool{

    let s = s.to_lowercase();
    let bytes = s.as_bytes();
    let mut begin = 0;
    let mut end = bytes.len()-1;
    return loop {
        if begin >= end {
            break true;
        }
        if bytes[begin] < 'a' as u8 || bytes[begin] > 'z' as u8 {
            begin += 1;
            continue;
        }

        if bytes[end] < 'a' as u8 || bytes[end] > 'z' as u8 {
            end -= 1;
            continue;
        }

        if bytes[begin] == bytes[end] {
            begin += 1;
            end -= 1;
        } else {
            break false;
        }
    }
}


#[cfg(test)]
mod test {
    use crate::lc::algo_125::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        println!("{}", is_palindrome("OP".to_string()));
    }
}