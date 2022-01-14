use std::i8::MIN;

fn coin_change(coins: Vec<i32>, amount: i32) -> (i32, Vec<Vec<i32>>){
    let MIN_VAL = 9999999;
    let mut dp = vec![vec![MIN_VAL; (amount+1) as usize]; coins.len()];
    for i in 0..coins.len() {
        for j in 0..(amount as usize + 1) {
            if j == 0 {
                dp[i][j] = 0;
            } else if i == 0 {
                if j as i32 % coins[0] == 0 {
                    dp[i][j] = j as i32 / coins[0];
                }
            }
        }
    }


    for i in 1..coins.len() {
        for j in 1..(amount as usize + 1) {
            let mut minCoinNum = MIN_VAL;
            let mut k = 0;
            while (j as i32 - k * coins[i]) >= 0 {
                let curCoinNum = dp[i-1][j - (k * coins[i]) as usize] + k;
                minCoinNum = if curCoinNum < minCoinNum {
                    curCoinNum
                } else {
                    minCoinNum
                };
                k += 1
            }

            dp[i][j] = minCoinNum;
        }
    }

    return (dp[coins.len()-1][amount as usize], dp)
}


#[cfg(test)]
mod test {
    use crate::other::algo_322::coin_change;

    #[test]
    fn testCoinChange() {
        println!("{:?}", coin_change(vec![1, 2, 3], 5));
    }
}