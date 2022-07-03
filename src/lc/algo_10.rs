
pub fn is_match(s: String, p: String) -> bool {
    let mut dp = vec![vec![false; p.len()+1]; s.len()+1];
    dp[0][0] = true;
    for i in 1..=s.len() {
        dp[i][0] = false;
    }

    for i in 1..=p.len() {
        if &p[(i-1)..i] == "*" {
            dp[0][i] = dp[0][i-2];
        } else {
            dp[0][i] = false;
        }
    }

    for i in 0..s.len() {
        for j in 0..p.len() {
            if &p[j..j+1] != "*" {
                dp[i+1][j+1] = if &s[i..i+1] == &p[j..j+1] || &p[j..j+1] == "." {
                    dp[i][j]
                } else {
                    false
                };
            } else {
                dp[i+1][j+1] = if &s[i..i+1] == &p[j-1..j] || &p[j-1..j] == "."{
                    dp[i+1][j-1] || dp[i][j+1]

                } else {
                    dp[i+1][j-1]
                };
            }
        }
    }

    dp[s.len()][p.len()]
}

#[cfg(test)]
mod test {
    use crate::lc::algo_10::is_match;

    #[test]
    pub fn test() {
        println!("{}", is_match("aa".to_string(), "a*".to_string()));
    }
}