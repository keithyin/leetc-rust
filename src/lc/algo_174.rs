use std::cmp::min;

pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let tot_row = dungeon.len();
    let tot_col = dungeon[0].len();
    let mut dp = vec![vec![(0, 0); tot_col]; tot_row];
    dp[0][0] = (dungeon[0][0], dungeon[0][0]);
    for row in 1..tot_row {
        let cur_hp = dp[row-1][0].0 + dungeon[row][0];
        dp[row][0] = (cur_hp, min(cur_hp, dp[row-1][0].1))
    }
    for col in 1..tot_col {
        let cur_hp = dp[0][col-1].0 + dungeon[0][col];
        dp[0][col] = (cur_hp, min(cur_hp, dp[0][col-1].1));
    }

    for row in 1..tot_row {
        for col in 1..tot_col {
            let which_pre = if dp[row-1][col].0 < dp[row][col-1].0 {
                dp[row][col-1]
            } else if dp[row-1][col].0 > dp[row][col-1].0 {
                dp[row-1][col]
            } else {
                if dp[row-1][col].1 > dp[row][col-1].1 {
                    dp[row-1][col]
                } else {
                    dp[row][col-1]
                }
            };
            let cur_hp = which_pre.0 + dungeon[row][col];
            dp[row][col] = (cur_hp, min(cur_hp, which_pre.1));
        }
    }

    dp[tot_row-1][tot_col-1].1

}