
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    if heights.is_empty() {
        return 0;
    }
    let length = heights.len() as i32;
    let mut cursor = 0;
    let mut max_area = 0;
    while cursor < length {
        let cur_height = heights[cursor as usize];
        let mut right_cursor = cursor;
        let mut left_cursor = cursor;
        while right_cursor >= 0 && cur_height <= heights[right_cursor as usize] {
            right_cursor -= 1;
        }
        while left_cursor < length && cur_height <= heights[left_cursor as usize] {
            left_cursor += 1;
        }
        let span = (left_cursor - right_cursor - 1) ;
        max_area = if max_area < (span * cur_height) {
            span * cur_height
        } else {
            max_area
        };
        cursor += 1;
    }
    max_area
}

#[cfg(test)]
mod test {
    use crate::stack::algo_84::largest_rectangle_area;

    #[test]
    fn test_largest_rectangle_area() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        println!("{}", largest_rectangle_area(heights));
    }
}