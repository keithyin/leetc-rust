use std::cmp::max;
use std::collections::HashMap;

// status: 0，当前手中无彩票。status:1 当前手中有彩票
pub fn max_profit_core(prices: &Vec<i32>, cur_pos: usize, status: usize, left_time: usize, memo: &mut HashMap<(usize, usize, usize), i32>) -> i32 {
    if left_time == 0 {
        return 0;
    }
    if cur_pos >= prices.len() {
        return 0;
    }

    if let Some(res) = memo.get(&(cur_pos, status, left_time)) {
        return *res;
    }

    let profit = if status == 0 {
        let today_buy = -prices[cur_pos] + max_profit_core(prices, cur_pos+1, 1, left_time, memo);
        let today_not_buy = max_profit_core(prices, cur_pos+1, status, left_time, memo);
        max(today_buy, today_not_buy)
    } else {
        let today_sell = prices[cur_pos] + max_profit_core(prices, cur_pos+1, 0, left_time-1, memo);
        let today_not_sell = max_profit_core(prices, cur_pos+1, 1, left_time, memo);
        max(today_sell, today_not_sell)
    };
    memo.insert((cur_pos, status, left_time), profit);
    profit
}



pub fn max_profit(prices: Vec<i32>) -> i32 {
    max_profit_core(&prices, 0, 0, 2, &mut HashMap::new())
}