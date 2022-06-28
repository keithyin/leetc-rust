
fn word_break_core(s: &str, word_dict: &Vec<String>, tracer: &mut Vec<String>, results: &mut Vec<String>) {
    if s.len() == 0 {
        results.push(tracer.clone().join(" "));
        return;
    }

    for word in word_dict.iter() {
        if s.starts_with(word.as_str()) {
            tracer.push(word.clone());
            word_break_core(&s[word.len()..], word_dict, tracer, results);
            tracer.pop();
        }
    }
}

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut results = vec![];
    let mut tracer = vec![];
    word_break_core(s.as_str(), &word_dict, &mut tracer, &mut results);
    results
}

#[cfg(test)]
mod test {

    #[test]
    fn test_word_break() {
        println!("{}", &("hello"[0..2]))
    }
}