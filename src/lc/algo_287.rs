
// pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
//
//     for v in nums.iter_mut() {
//         *v -= 1;
//     }
//     let mut i = 0;
//     while i < nums.len() as i32 {
//         if nums[i as usize] != i {
//             if nums[nums[i as usize] as usize] == nums[i as usize] {
//                 return nums[i as usize] + 1;
//             }
//             nums.swap(i as usize, nums[i as usize] as usize);
//         } else {
//             i += 1
//         }
//     }
//     0
// }