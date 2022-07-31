use std::cmp::max;

pub fn multiply(num1: String, num2: String) -> String {
    let zero = '0' as u8;
    let num1 = num1.as_str().as_bytes();
    let num2 = num2.as_str().as_bytes();
    let num1_len = num1.len() as i32;
    let num2_len = num2.len() as i32;
    let mut values = vec![];
    let mut string_max_len = 0;
    for i in (0..num1_len as usize).rev() {
        let num1_cur = (num1[i] - zero) as i32;
        let mut carry = 0;
        let mut level1_res = vec![0; num1_len as usize -i-1];
        for j in (0..num2_len as usize).rev() {
            let num2_cur = (num2[j] - zero) as i32;
            let mut level2_res = num1_cur * num2_cur + carry;
            carry = level2_res / 10;
            level2_res = level2_res % 10;
            level1_res.push(level2_res);
        }
        if carry > 0 {
            level1_res.push(carry);
        }
        string_max_len = max(string_max_len, level1_res.len());
        values.push(level1_res);
    }

    let mut carry = 0;
    let mut result = vec![];
    for pos in 0..string_max_len {
        let mut cur_res = 0;
        for single_values in values.iter() {
            cur_res += (if pos < single_values.len() {single_values[pos]} else {0});
        }


        cur_res += carry;
        carry = cur_res / 10;
        cur_res = cur_res % 10;
        result.push(cur_res);
    }
    if carry > 0 {
        result.push(carry);
    }


    let result: Vec<String> = result.iter().rev().map(|x| x.to_string()).collect();

    let result = result.join("");
    return if result.starts_with("0") {
        "0".to_string()
    } else {
        result
    };

}

#[cfg(test)]
mod test {
    use crate::lc::algo_43::multiply;

    #[test]
    pub fn test_multiply() {
        println!("{}", multiply("44".to_string(), "0".to_string()));
    }
}