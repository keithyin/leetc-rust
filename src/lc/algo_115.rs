use std::collections::HashMap;

pub fn num_distinct_core(s: &str, t: &str, pos: (usize, usize), memo: &mut HashMap<(usize, usize), i32>) -> i32 {
    if t.len() == pos.1 {
        return 1;
    }

    if s.len() == pos.0 {
        return 0;
    }
    if let Some(cnt) = memo.get(&pos) {
        return *cnt;
    }
    let choose_cur = if &s[pos.0..pos.0+1] == &t[pos.1..pos.1+1] {
        num_distinct_core(s, t, (pos.0+1, pos.1+1), memo)
    } else {
        0
    };

    let not_choose = num_distinct_core(s, t, (pos.0+1, pos.1), memo);
    memo.insert(pos, choose_cur + not_choose);
    choose_cur + not_choose

}

pub fn num_distinct(s: String, t: String) -> i32 {
    num_distinct_core(s.as_str(), t.as_str(), (0, 0), &mut HashMap::new())
}