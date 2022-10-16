
/*
1,2,3,4
6

第k小值, 3
1=k/2
2,6: 1 进。
k=2
1=k/2
3,6: 3进
k=1
0=k/2


 */
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let tot_len = nums1.len() + nums2.len();
    let

    let mut begin1 = 0;
    let mut begin2 = 0;

    0.
}

/*

 */
pub fn find_kth_small_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: i32) -> i32 {
    let mut begin1 = 0;
    let mut begin2 = 0;

    while k > 0 {
        let mid = (k as usize) / 2;
        let mid1 = if mid < nums1.len() {mid} else {nums1.len() - 1};
        let mid2 = if mid < nums2.len() {mid} else {nums2.len() - 1};
        if nums1[mid1] < nums2[mid2] {
            k -= (mid1 - begin1 + 1);
            begin1 = begin1 + mid + 1;
        } else if nums1[mid1] > nums2[mid2] {
            k -= (mid2 - begin2 + 1);
            begin2 = begin2 + mid2 + 1;
        } else {

        }

    }
    0
}