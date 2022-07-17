
pub fn num_trees(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        let mut count = 0;
        for root in 1..=i {
            let left_num = root - 1;
            let right_num = i - root;
            count += (dp[left_num as usize] * dp[right_num as usize]);
        }
        dp[i as usize] = count;
    }

    dp[n as usize]
}

#[cfg(test)]
mod test {
    use crate::lc::algo_96::num_trees;

    #[test]
    pub fn test_num_trees() {
        println!("{}", num_trees(3));
    }
}