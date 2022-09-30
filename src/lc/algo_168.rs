
pub fn convert_to_title(mut column_number: i32) -> String {
    let mut stack = vec![];

    while column_number > 0 {
        column_number -= 1;
        let mod_res = column_number % 26;
        stack.push('A' as u32 + mod_res as u32);
        column_number = column_number / 26;
    }
    let mut res = "".to_string();
    while let Some(v) = stack.pop() {
        res.push(char::from_u32(v).unwrap());
    }

    res
}