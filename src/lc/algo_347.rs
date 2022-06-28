use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counter = HashMap::new();
    for v in &nums {
        if counter.contains_key(v) {
            counter.insert(*v, counter[v] + 1);
        } else {
            counter.insert(*v, 1);
        }
    }
    let mut pairs = vec![];
    for (val, count) in &counter {
        pairs.push((*val, *count));
    }

    let mut begin_pos = 0;
    let mut end_pos = pairs.len()-1;
    loop {
        let pos = split(&mut pairs, begin_pos, end_pos);
        if pos == k as usize {
            break;
        }
        if pos > k as usize {
            end_pos = pos - 1;
        } else {
            begin_pos = pos + 1;
        }
    }

    let mut res: Vec<i32> = vec![];
    for i in 0..k {
        res.push(pairs[i as usize].0);
    }
    res

}


fn split(nums: &mut Vec<(i32, i32)>, begin_pos: usize, end_pos: usize) -> usize {
    if begin_pos >= end_pos {
        return begin_pos;
    }
    let anchor = nums[begin_pos];
    let mut left_cursor = begin_pos as i32;
    let mut right_cursor = end_pos as i32;

    while left_cursor < right_cursor {
        while left_cursor < right_cursor && nums[right_cursor as usize].1 < anchor.1 {
            right_cursor -= 1;
        }

        if left_cursor < right_cursor {
            nums.swap(left_cursor as usize, right_cursor as usize);
            left_cursor += 1;
        }

        while left_cursor < right_cursor && nums[left_cursor as usize].1 >= anchor.1 {
            left_cursor += 1;
        }

        if left_cursor < right_cursor {
            nums.swap(left_cursor as usize, right_cursor as usize);
            right_cursor -= 1;
        }

    }

    nums[left_cursor as usize] = anchor;
    left_cursor as usize
}
