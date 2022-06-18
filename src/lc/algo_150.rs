use std::str::FromStr;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut operands = vec![];
    for str in tokens.iter() {
        match str.as_str() {
            "+" => {
                let second = operands.remove(operands.len()-1);
                let first = operands.remove(operands.len()-1);
                operands.push(first + second);
            }
            "-" => {
                let second = operands.remove(operands.len()-1);
                let first = operands.remove(operands.len()-1);
                operands.push(first - second);
            }
            "*" => {
                let second = operands.remove(operands.len()-1);
                let first = operands.remove(operands.len()-1);
                operands.push(first * second);
            }
            "/" => {
                let second = operands.remove(operands.len()-1);
                let first = operands.remove(operands.len()-1);
                operands.push(first / second);
            }
            _ => operands.push(i32::from_str(str.as_str()).unwrap()),
        }
    }
    operands[0]
}

#[cfg(test)]
mod test {
    use crate::lc::algo_150::eval_rpn;

    #[test]
    fn test_eval_rpn() {
        println!("{}", eval_rpn(vec!["1".to_string(), "2".to_string(), "-".to_string()]));
    }
}