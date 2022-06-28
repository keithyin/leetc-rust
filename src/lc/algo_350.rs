use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut counter1: HashMap<i32, u32> = HashMap::new();
    let mut counter2: HashMap<i32, u32> = HashMap::new();

    for v in nums1.iter() {
        if counter1.contains_key(v) {
            counter1.insert(*v, counter1[v] + 1);
        } else {
            counter1.insert(*v, 1);
        }
    }

    for v in nums2.iter() {
        if counter2.contains_key(v) {
            counter2.insert(*v, counter2[v] + 1);
        } else {
            counter2.insert(*v, 1);
        }
    }
    let mut res = vec![];
    for (k, v) in &counter1 {
        if counter2.contains_key(k) {
            let dup_num = if *v < counter2[k] {
                *v
            } else {
                counter2[k]
            };

            for i in 0..dup_num {
                res.push(*k);
            }

        }
    }

    res

}

#[cfg(test)]
mod test {
    use crate::lc::algo_350::intersect;

    #[test]
    fn test_intersect() {
        let nums1 = vec![];
        let nums2 = vec![];
        intersect(nums1, nums2);
    }
}