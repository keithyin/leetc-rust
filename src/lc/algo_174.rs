use std::cmp::{max, min};

pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let tot_row = dungeon.len();
    let tot_col = dungeon[0].len();
    let mut dp = vec![vec![0; tot_col]; tot_row];
    dp[tot_row-1][tot_col-1] = max(-dungeon[tot_row-1][tot_col-1], 1);
    for row in (0..tot_row-1).rev() {
        dp[row][tot_col-1] = max(dp[row+1][tot_col-1] - dungeon[row][tot_col-1], 1);
    }
    for col in (0..tot_col-1).rev() {
        dp[tot_row-1][col] = max(dp[tot_row-1][col+1] - dungeon[tot_row-1][col], 1);
    }

    for row in (0..tot_row-1).rev() {
        for col in (0..tot_col-1).rev() {
            dp[row][col] = max(min(dp[row][col+1], dp[row+1][col]) - dungeon[row][col], 1);
        }
    }

    dp[0][0]

}

#[cfg(test)]
mod test {
    use crate::lc::algo_174::calculate_minimum_hp;

    #[test]
    pub fn test_cal_min() {
        let dungeon = vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]];

        println!("{}", calculate_minimum_hp(dungeon));
    }
}