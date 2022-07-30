
pub fn find_substring_core(s: &str, words: &mut Vec<String>) ->bool {
    if words.len() == 0  {
        return true;
    }

    let mut result = false;
    let words_num = words.len();
    for i in 0..words_num {
        if s.starts_with(&words[i]) {
            let word = words.remove(i);
            result = find_substring_core(&s[word.len()..], words);
            words.insert(i, word);
            if result {
                break;
            }
        }

    }
    result

}

pub fn find_substring(s: String, mut words: Vec<String>) -> Vec<i32> {
    let s = s.as_str();
    let mut results = vec![];
    for i in 0..s.len() {
        if find_substring_core(&s[i..], &mut words) {
            results.push(i as i32);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use crate::lc::algo_30::find_substring;

    #[test]
    pub fn test() {
        let results = find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]);
        println!("{:?}", results);
    }

    #[test]
    pub fn test_str() {
        println!("{}", "hello".starts_with(&"he".to_string()));
    }
}