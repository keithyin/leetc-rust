
pub fn is_match(s: String, p: String) -> bool {
    let mut dp = vec![vec![false; p.len()+1]; s.len()+1];
    dp[0][0] = true;

    for i in 1..=s.len() {
        dp[i][0] = false;
    }
    for j in 1..=p.len() {
        if &p[(j-1)..j] == "*" {
            dp[0][j] = dp[0][j-1];
        } else {
            dp[0][j] = false;
        }
    }

    let s = s.as_str();
    let p = p.as_str();
    for i in 0..s.len() {
        for j in 0..p.len() {
            if &p[j..j+1] == "*" {
                dp[i+1][j+1] = dp[i+1][j] || dp[i][j+1];
            } else {
                dp[i+1][j+1] = if &s[i..i+1] == &p[j..j+1] || &p[j..j+1] == "?" {
                    dp[i][j]
                } else {
                    false
                };
            }
        }
    }
    dp[s.len()][p.len()]
}

#[cfg(test)]
mod test {
    use crate::lc::algo_44::is_match;

    #[test]
    pub fn test() {
        println!("{}", is_match("aa".to_string(), "*".to_string()));
    }
}
