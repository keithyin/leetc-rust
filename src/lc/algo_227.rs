use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::str::FromStr;

pub fn calculate(s: String) -> i32 {
    let mut priority_map = HashMap::new();
    priority_map.insert("*", 2);
    priority_map.insert("/", 2);
    priority_map.insert("+", 1);
    priority_map.insert("-", 1);


    let mut operands_queue: Vec<String> = vec![];
    let mut operators_queue: Vec<String> = vec![];

    for item in token_split(s).into_iter() {

        println!("operands: {:?}", operands_queue);
        println!("operators: {:?}", operators_queue);

        if priority_map.contains_key(item.as_str()) {
            while let Some(last_operator) = operators_queue.last() {
                if priority_map[item.as_str()] <= priority_map[last_operator.as_str()] {
                    let last_operator = operators_queue.remove(operators_queue.len()-1);
                    let operand2 = operands_queue.remove(operands_queue.len()-1);
                    let operand1 = operands_queue.remove(operands_queue.len()-1);
                    operands_queue.push(compute(&operand1, &operand2, &last_operator).to_string());
                } else {
                    break;
                }
            }
            operators_queue.push(item);

        } else {
            operands_queue.push(item);
        }
    }

    for operator in operators_queue.iter().rev() {

        println!("operands: {:?}", operands_queue);
        println!("operators: {:?}", operators_queue);

        let operand2 = operands_queue.remove(operands_queue.len()-1);
        let operand1 = operands_queue.remove(operands_queue.len()-1);
        operands_queue.push(compute(&operand1, &operand2, operator).to_string());
    }

    i32::from_str(operands_queue[0].as_str()).unwrap()

}

fn token_split(mut s: String) -> Vec<String> {

    let mut operators = HashSet::new();
    operators.insert('*' as u8);
    operators.insert('/' as u8);
    operators.insert('+' as u8);
    operators.insert('-' as u8);
    let mut tokens = vec![];

    s = s.replace( " ", "");
    let mut last_idx = 0;

    for i in 0..s.len(){
        if operators.contains(&s.as_bytes()[i]) {
            tokens.push(String::from(&s[last_idx..i]));
            tokens.push(String::from(&s[i..(i+1)]));
            last_idx = i + 1;
        }
    }
    tokens.push(s.chars().skip(last_idx).take(s.len() - last_idx).collect());

    tokens
}

fn compute(operand1: &String, operand2: &String, operator: &String) -> i32{
    match operator.as_str() {
        "+" => i32::from_str(operand1.as_str()).unwrap() +  i32::from_str(operand2.as_str()).unwrap(),
        "-" => i32::from_str(operand1.as_str()).unwrap() -  i32::from_str(operand2.as_str()).unwrap(),
        "*" => i32::from_str(operand1.as_str()).unwrap() *  i32::from_str(operand2.as_str()).unwrap(),
        "/" => i32::from_str(operand1.as_str()).unwrap() /  i32::from_str(operand2.as_str()).unwrap(),
        _ => panic!(format!("invalid operator")),
    }
}


#[cfg(test)]
mod test {
    use crate::lc::algo_227::calculate;

    #[test]
    fn test_calculate() {
        println!("{}", calculate("1*2-3/4+5*6-7*8+9/10".to_string()));
    }
}