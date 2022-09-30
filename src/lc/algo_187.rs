use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }
    let s = s.as_str();
    let mut counter = HashMap::new();
    for i in 0..s.len()-9 {
        let sub_s = String::from(&s[i..i+10]);
        if counter.contains_key(&sub_s) {
            let c = counter[&sub_s]+1;
            counter.insert(sub_s, c);
        } else {
            counter.insert(sub_s, 1);
        }
    }
    let mut result = vec![];
    let ref_result: Vec<&String> = counter.iter().filter(|(s, i)| **i > 1).map(|(s, i)| s).collect();
    for v in ref_result.iter() {
        result.push(String::clone(*v));
    }
    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_187::find_repeated_dna_sequences;

    #[test]
    pub fn test() {
        let s = "AAAAAAAAAAA".to_string();
        println!("{:?}", find_repeated_dna_sequences(s));
    }
}