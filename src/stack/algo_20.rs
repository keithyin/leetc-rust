
fn valid_bracket(brackets: &String) -> bool{
    let mut stack: Vec<char> = Vec::new();
    for item in brackets.chars() {
        match item {
            '(' | '{' | '[' => stack.push(item),
            _ => {
                if stack.len() > 0 {
                    if let Some(top) = stack.get(stack.len() - 1) {
                        let item_ascii = item.to_string().as_bytes()[0];
                        let top_ascii = (*top).to_string().as_bytes()[0];
                        if (item_ascii as i32 - top_ascii as i32) > 1 {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    if stack.len() == 0 {
        return true;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_bracket() {
        let mut brackets= String::from("((()))}");
        assert!(!valid_bracket(&brackets));
        brackets = String::from("((()))");
        assert!(valid_bracket(&brackets));
    }
}