pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut left_min = vec![0; heights.len()];
    let mut right_min = vec![0; heights.len()];

    let mut stack = vec![];
    for i in (0..heights.len()).rev() {
        while let Some(inner) = stack.last(){
            if heights[i] <= heights[*inner] {
                stack.remove(stack.len() - 1);
            } else {
                break;
            }
        }

        if let Some(inner) = stack.last() {
            right_min[i] = *inner as i32;
        } else {
            right_min[i] = -1;
        }
        stack.push(i);
    }

    stack = vec![];
    for i in 0..heights.len() {
        while let Some(inner) = stack.last(){
            if heights[i] <= heights[*inner] {
                stack.remove(stack.len() - 1);
            } else {
                break;
            }
        }

        if let Some(inner) = stack.last() {
            left_min[i] = (*inner) as i32;
        } else {
            left_min[i] = -1;
        }
        stack.push(i);
    }


    let mut max_area = 0;
    for i in 0..heights.len() {
        let left = left_min[i] + 1;
        let right = if right_min[i] == -1 {
            heights.len() as i32 - 1
        } else {
            right_min[i] - 1
        };

        max_area = if max_area > heights[i] * (right - left + 1) {
            max_area
        } else {
            heights[i] * (right - left + 1)
        }
    }

    max_area
}

#[cfg(test)]
mod test {
    use crate::lc::algo_84::largest_rectangle_area;

    #[test]
    pub fn test_area() {
        println!("{}", largest_rectangle_area(vec![2,4]));
    }
}