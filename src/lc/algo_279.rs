use std::cmp::min;

pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![0; (n+1) as usize];
    for i in 1..n+1 {
        let mut j = 1;
        let mut minn = i32::MAX;
        while j * j <= i {
            minn = min(minn, 1 + dp[(i - j * j) as usize]);
            j += 1;
        }
        dp[i] = minn;
    }

    dp[n]

}