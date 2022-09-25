
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut begin = 0 as i32;
    let mut end = numbers.len() as i32 - 1;
    while begin < end {
        let val = numbers[begin as usize] + numbers[end as usize];
        if val == target {
            return vec![begin+1, end+1];
        } else if val > target {
            end -= 1;
        } else {
            begin += 1;

        }

    }
    vec![-1, -1]
}