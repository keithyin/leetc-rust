use std::collections::HashMap;

pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let mut res = 0;
    for n1 in nums1.iter() {
        for n2 in nums2.iter() {
            let key = -(n1 + n2);
            if counter.contains_key(&key) {
                counter.insert(key, counter[&key] + 1);
            } else {
                counter.insert(key, 1);
            }
        }
    }

    for n3 in nums3.iter() {
        for n4 in nums4.iter() {
            let key = n3 + n4;
            res += if counter.contains_key(&key) {
                counter[&key]
            } else {
                0
            };
        }

    }

    res
}

#[cfg(test)]
mod test {
    use crate::lc::algo_454::four_sum_count;

    #[test]
    fn test_four_sum_count() {
        let nums1 = vec![1, 2, 3, 4];
        let nums2 = vec![-1, 2, 3, 4];
        let nums3 = vec![1, -2, 3, 4];
        let nums4 = vec![-1, 2, 3, 4];

        println!("{}", four_sum_count(nums1, nums2, nums3, nums4))

    }
}