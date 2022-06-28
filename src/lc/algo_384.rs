
use rand::Rng;

pub fn shuffle(mut nums:Vec<i32>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    for i in 1..nums.len() {
        let pos = rng.gen_range(0..=i);
        let tmp = nums[i];
        nums[i] = nums[pos];
        nums[pos] = tmp;
    }
    return nums;

}