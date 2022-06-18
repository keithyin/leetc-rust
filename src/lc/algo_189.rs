use std::collections::HashSet;

pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {

    k %= (nums.len() as i32);
    if k == 0 {
        return;
    }
    let mut visited = HashSet::new();
    for begin_pos in 0..nums.len() {
        if visited.contains(&begin_pos) {
            break;
        }
        visited.insert(begin_pos);
        let mut next_pos = (begin_pos + k as usize) % (nums.len());
        let mut pre_v = nums[begin_pos];
        while next_pos != begin_pos {
            visited.insert(next_pos);
            let next_v = nums[next_pos];
            nums[next_pos] = pre_v;
            pre_v = next_v;
            next_pos = (next_pos + k as usize) % (nums.len());
        }
        nums[next_pos] = pre_v;
    }

}

#[cfg(test)]
mod test {
    use crate::lc::algo_189::rotate;

    #[test]
    fn test_rotate() {
        let mut nums = vec![0, 1, 2, 3];
        rotate(&mut nums, 2);
        println!("{:?}", nums);
    }
}
