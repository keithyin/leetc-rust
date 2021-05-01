use std::str::FromStr;

fn compute_value(tokens: Vec<&str>) -> i32 {
    let mut value_stack: Vec<String> = vec![];
    for token in tokens.iter() {
        match *token {
            "+" => {
                if let (Some(second_operand), Some(first_operand)) = (value_stack.pop(), value_stack.pop()) {
                    let second_operand = i32::from_str(&second_operand).unwrap();
                    let first_operand = i32::from_str(&first_operand).unwrap();
                    value_stack.push((first_operand + second_operand).to_string());
                }
            },
            "-" => {
                if let (Some(second_operand), Some(first_operand)) = (value_stack.pop(), value_stack.pop()) {
                    let second_operand = i32::from_str(&second_operand).unwrap();
                    let first_operand = i32::from_str(&first_operand).unwrap();
                    value_stack.push((first_operand - second_operand).to_string());
                }
            },
            "*" => {
                if let (Some(second_operand), Some(first_operand)) = (value_stack.pop(), value_stack.pop()) {
                    let second_operand = i32::from_str(&second_operand).unwrap();
                    let first_operand = i32::from_str(&first_operand).unwrap();
                    value_stack.push((first_operand * second_operand).to_string());
                }
            },
            "/" => {
                if let (Some(second_operand), Some(first_operand)) = (value_stack.pop(), value_stack.pop()) {
                    let second_operand = i32::from_str(&second_operand).unwrap();
                    let first_operand = i32::from_str(&first_operand).unwrap();
                    value_stack.push((first_operand / second_operand).to_string());
                }
            },
            _ => {
                value_stack.push(String::from(*token));
            },
        }
    }
    let result = i32::from_str(&value_stack[0]).unwrap();
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_value() {
        let tokens = vec!["1", "2", "+", "4", "/"];
        assert_eq!(compute_value(tokens), 0);
        let tokens = vec!["1", "2", "+", "4", "/", "10", "+", "100", "*"];
        assert_eq!(compute_value(tokens), 1000);
    }
}