use std::collections::{HashMap};

pub fn is_interleave_core(s1: &str, s2: &str, s3: &str, pos: (usize, usize, usize), memo: &mut HashMap<(usize, usize, usize), bool>) -> bool{

    if let Some(res) = memo.get(&pos) {
        return *res;
    }

    if pos.2 >= s3.len(){
        return true;
    }
    let cur_s3_byte = &s3[pos.2..pos.2+1];
    let cur_s2_byte = if pos.1 < s2.len() {
        &s2[pos.1..pos.1+1]
    } else {
        ""
    };
    let cur_s1_byte = if pos.0 < s1.len() {
        &s1[pos.0..pos.0+1]
    } else {
        ""
    };
    let mut result = false;
    if cur_s3_byte == cur_s2_byte {
        result = is_interleave_core(s1, s2, s3, (pos.0, pos.1+1, pos.2+1), memo);
        memo.insert((pos.0, pos.1+1, pos.2+1), result);
    }
    if result {
        return result;
    }

    if cur_s3_byte == cur_s1_byte {
        result = is_interleave_core(s1, s2, s3, (pos.0+1, pos.1, pos.2+1), memo);
        memo.insert((pos.0+1, pos.1, pos.2+1), result);
    }
    result
}



pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {

    let mut memo = HashMap::new();

    return if s3.len() == (s1.len() + s2.len()) {
        is_interleave_core(s1.as_str(), s2.as_str(), s3.as_str(), (0, 0, 0), &mut memo)
    } else {
        false
    };
}
