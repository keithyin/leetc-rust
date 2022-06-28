
fn partition_core(s: &str, palindrome_dp: &Vec<Vec<bool>>, begin_pos: usize,
                  tracer: &mut Vec<String>, results: &mut Vec<Vec<String>>) {

    if begin_pos == s.len() {
        results.push(tracer.clone());
        return;
    }

    for j in begin_pos..s.len() {
        if palindrome_dp[begin_pos][j] {
            tracer.push(s[begin_pos..j+1].to_string());

            partition_core(s, palindrome_dp, j+1, tracer, results);

            tracer.pop();
        }
    }
}

pub fn partition(s: String) -> Vec<Vec<String>> {
    //  s[i][j] i -> j is 回文？
    let mut dp = vec![vec![false; s.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][i] = true;
    }

    for span in 1..s.len() {
        for begin in 0..s.len() {
            let end = begin + span;
            if end >= s.len() {
                break;
            }
            if &s[begin..begin+1] == &s[end..end+1] {
                if span == 1 {
                    dp[begin][end] = true;
                } else {
                    dp[begin][end] = dp[begin+1][end-1];
                }
            }
        }
    }

    let mut tracer = vec![];
    let mut results = vec![];
    partition_core(s.as_str(), &dp, 0, &mut tracer, &mut results);
    results

}

#[cfg(test)]
mod test {
    use crate::lc::algo_131::partition;

    #[test]
    fn test_partition() {

        let results = partition("aab".to_string());
        println!("{:?}", results);

    }
}