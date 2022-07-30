use std::cmp::max;

pub fn multiply(num1: String, num2: String) -> String {
    let zero = '0' as u8;
    let num1 = num1.as_str().as_bytes();
    let num2 = num2.as_str().as_bytes();
    let num1_len = num1.len() as i32;
    let num2_len = num2.len() as i32;
    let mut value_strings = vec![];
    let mut string_max_len = 0;
    for i in (0..num1_len as usize).rev() {
        let num1_cur = (num1[i] - zero) as i32;
        let mut carry = 0;
        let mut level1_res: Vec<String> = vec![0; num1_len as usize -i-1].iter().map(|x| x.to_string()).collect();
        let mut level1_res = level1_res.join("");
        for j in (0..num2_len as usize).rev() {
            let num2_cur = (num2[j] - zero) as i32;
            let mut level2_res = num1_cur * num2_cur + carry;
            carry = level2_res / 10;
            level2_res = level2_res % 10;
            level1_res.push_str(level2_res.to_string().as_str());
        }
        if carry > 0 {
            level1_res.push_str(carry.to_string().as_str());
        }
        string_max_len = max(string_max_len, level1_res.len());
        value_strings.push(level1_res);
    }

    for pos in 0..string_max_len {
        for value_str in value_strings.iter() {

        }
    }


}

#[cfg(test)]
mod test {
    use crate::lc::algo_43::multiply;

    #[test]
    pub fn test_multiply() {
        println!("{}", multiply("4".to_string(), "25".to_string()));
    }
}