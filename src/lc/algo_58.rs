
pub fn length_of_last_word(s: String) -> i32 {
    let mut last_word_len = 0;
    let s = s.as_str();
    let mut pre_byte = " ";

    for idx in 0..s.len() {
        let cur_byte = &s[idx..idx+1];
        if pre_byte == " " {
            if cur_byte != " " {
                last_word_len = 1;
            }
        } else {
            if cur_byte != " "{
                last_word_len += 1;
            }
        }
        pre_byte = cur_byte;
    }
    last_word_len
}

#[cfg(test)]
mod test {
    use crate::lc::algo_58::length_of_last_word;

    #[test]
    pub fn test() {
        println!("{:?}", length_of_last_word("   fly me   to   the moon  ".to_string()));
    }

}