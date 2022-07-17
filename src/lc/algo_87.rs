use std::collections::HashMap;


// 1: match, 2, not match, 3: not sure
pub fn match_core(s1: &str, s2: &str, s1_b_e: (usize, usize), s2_b_e: (usize, usize)) -> i32 {
    if &s1[s1_b_e.0..s1_b_e.1] == &s2[s2_b_e.0..s2_b_e.1] {
        return 1;
    }
    let mut s1_counter = HashMap::new();
    for i in s1_b_e.0..s1_b_e.1 {
        if !s1_counter.contains_key(&s1[i..i+1]) {
            s1_counter.insert(&s1[i..i+1], 0);
        }
        s1_counter.insert(&s1[i..i+1], s1_counter[&s1[i..i+1]] + 1);
    }

    let mut s2_counter = HashMap::new();
    for i in s2_b_e.0..s2_b_e.1 {
        if !s2_counter.contains_key(&s2[i..i+1]) {
            s2_counter.insert(&s2[i..i+1], 0);
        }
        s2_counter.insert(&s2[i..i+1], s2_counter[&s2[i..i+1]] + 1);
    }

    if s1_counter.len() != s2_counter.len() {
        return 2;
    }

    for (k, v) in s1_counter {
        if let Some(s2_v) = s2_counter.get(k) {
            if *s2_v != v {
                return 2;
            }
        } else {
            return 2;
        }
    }

    return 3;
}

pub fn is_scramble_core(s1: &String, s2: &String, s1_b_e: (usize, usize), s2_b_e: (usize, usize), memo: &mut HashMap<(usize, usize, usize, usize), bool>) -> bool{

    let mut is_match = false;
    for len in 0..(s1_b_e.1 - s1_b_e.0) {
        let new_s1_b_e = (s1_b_e.0, s1_b_e.0+len);
        let new_s2_b_e = (s2_b_e.0, s2_b_e.0+len);
        let right_match = if let Some(res) = memo.get(&(new_s1_b_e.0, new_s1_b_e.1, new_s2_b_e.0, new_s2_b_e.1)) {
            *res as i32
        } else {
            match_core(s1, s2, new_s1_b_e, new_s2_b_e)
        };

        let new_s1_b_e = (s1_b_e.0+len, s1_b_e.1);
        let new_s2_b_e = (s2_b_e.0+len, s2_b_e.1);
        let left_match = if let Some(res) = memo.get(&(new_s1_b_e.0, new_s1_b_e.1, new_s2_b_e.0, new_s2_b_e.1)) {
            *res as i32
        } else {
            match_core(s1, s2, new_s1_b_e, new_s2_b_e)
        };

        if right_match == left_match && right_match == 1 {
            is_match = true;
            break;
        }





    }


    // if let Some(found) = memo.get(&(s1_b_e.0, s1_b_e.1, s2_b_e.0, s2_b_e.1)) {
    //
    // }

    true
}

pub fn is_scramble(s1: String, s2: String) -> bool {

    true
}

#[cfg(test)]
mod test {
    use crate::lc::algo_87::match_core;

    #[test]
    pub fn test_match_core() {
        let s1 = "hello";
        let s2 = "elloh";
        println!("{}", match_core(s1, s2, (0, s1.len()), (0, s2.len())));
    }
}