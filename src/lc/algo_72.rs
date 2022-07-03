use std::cmp::min;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![0; word2.len()+1]; word1.len()+1];
    for col in 1..=word2.len() {
        dp[0][col] = col as i32;
    }
    for row in 1..=word1.len() {
        dp[row][0] = row as i32;
    }
    let word1 = word1.as_str();
    let word2= word2.as_str();
    for i in 0..word1.len() {
        for j in 0..word2.len() {
            dp[i+1][j+1] = if &word1[i..i+1] == &word2[j..j+1] {
                dp[i][j]
            } else {
                min(min(1+dp[i][j], 1+dp[i][j+1]), 1+dp[i+1][j])
            };
        }
    }
    dp[word1.len()][word2.len()]
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {

    }
}