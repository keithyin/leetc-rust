pub fn min_array(numbers: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = numbers.len() as i32 - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if numbers[mid as usize] > numbers[high as usize] {
            low = mid+1;
        } else if numbers[mid as usize] < numbers[high as usize] {
            high = mid;
        } else {
            high -= 1;
        }
    }
    numbers[low as usize]
}
