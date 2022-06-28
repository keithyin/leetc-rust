pub fn num_decodings(s: String) -> i32 {
    let s: Vec<u8> = s.bytes().map(|x| x - '0' as u8).collect();
    let mut dp = vec![0; s.len()];
    for i in 0..s.len() {
        if i == 0 {
            if s[i] > 0 {
                dp[i] = 1;
            }
        } else {
            let cur_as_single = if s[i] == 0 {
                0
            } else {
                dp[i-1]
            };
            let cur_as_comp = if (s[i-1] * 10 + s[i]) > 0 && (s[i-1] * 10 + s[i]) <= 26 {
                dp[i-1]
            } else {
                0
            };

            dp[i] = cur_as_single + cur_as_comp;
        }
    }

    dp[dp.len()-1]
}

#[cfg(test)]
mod test {

    #[test]
    fn test_num_decodings() {
        println!("{}", '0' as u8);
    }
}