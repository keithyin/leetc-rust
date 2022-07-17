use std::cmp::min;
use std::collections::HashMap;

pub fn min_cut_core(tot_len: usize, cur_pos: usize, dp: &Vec<Vec<bool>>, memo: &mut HashMap<usize, i32>) -> i32 {
    if cur_pos >= tot_len {
        return 0;
    }
    if let Some(res) = memo.get(&cur_pos) {
        return *res;
    }
    let mut min_num_cut = i32::MAX;

    for end_pos in cur_pos..tot_len {
        if dp[cur_pos][end_pos] {
            let cur_cut = 1 + min_cut_core(tot_len, end_pos +1, dp, memo);
            min_num_cut = min(min_num_cut, cur_cut);
        }
    }
    memo.insert(cur_pos, min_num_cut);
    min_num_cut
}

pub fn min_cut(s: String) -> i32 {
    let s = s.as_str();
    let mut dp = vec![vec![false; s.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][i] = true;
    }

    for span in 1..s.len() {
        for begin in 0..(s.len()-span) {
            let end = begin + span;
            if span == 1 {
                if &s[begin..begin+1] == &s[end..end+1] {
                    dp[begin][end] = true;
                }
            }else {
                if &s[begin..begin+1] == &s[end..end+1] {
                    dp[begin][end] = dp[begin+1][end-1];
                }
            }
        }
    }

    min_cut_core(s.len(), 0, &dp, &mut HashMap::new()) - 1
}