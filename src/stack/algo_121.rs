
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
        return 0;
    }
    let mut max_value_after_tracer = vec![];
    max_value_after_tracer.resize(prices.len(), 0);
    let mut max_value_after = 0;
    let mut cursor = (prices.len()-1) as i32;
    while cursor >= 0 {
        let cur_val = prices[cursor as usize];
        if cur_val > max_value_after {
            max_value_after = cur_val;
        }
        max_value_after_tracer[cursor as usize] = max_value_after;
        cursor -= 1;
    }


    let mut max_val = 0;
    for i in 0..prices.len() {
        let cur_delta = max_value_after_tracer[i] - prices[i];
        max_val = if max_val < cur_delta {
            cur_delta
        } else {
            max_val
        };
    }

    max_val
}

#[cfg(test)]
mod test {
    use crate::stack::algo_121::max_profit;

    #[test]
    fn test_max_profit() {
        let prices = vec![1, 3, 7];
        assert_eq!(max_profit(prices), 6);
        let prices = vec![2, 3, 7, 1, 9];
        assert_eq!(max_profit(prices), 8);
    }
}