use std::cmp::max;
use std::collections::HashMap;

fn max_profit_core(prices: &Vec<i32>, pos: usize, has_stock: bool, cache: &mut HashMap<(usize, bool), i32>) -> i32 {
    if pos >= prices.len() {
        return 0;
    }
    return if has_stock {
        if let Some(v) = cache.get(&(pos, has_stock)) {
            *v
        } else {
            let today_sell = prices[pos] + max_profit_core(prices, pos + 1, false, cache);
            let today_not_sell = max_profit_core(prices, pos + 1, true, cache);
            let max_profit = max(today_sell, today_not_sell);
            cache.insert((pos, has_stock), max_profit);
            max_profit
        }
    } else {
        if let Some(v) = cache.get(&(pos, has_stock)) {
            *v
        } else {
            let today_buy = -prices[pos] + max_profit_core(prices, pos + 1, true, cache);
            let today_not_buy = max_profit_core(prices, pos + 1, false, cache);
            let max_profit = max(today_buy, today_not_buy);
            cache.insert((pos, has_stock), max_profit);
            max_profit
        }
    }

}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cache = HashMap::new();
    max_profit_core(&prices, 0, false, &mut cache)

}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_max_profit() {

    }
}