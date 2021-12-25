use std::cmp::min;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left_max = 0;
    let mut right_max = 0;
    let mut left_max_vec = vec![0; height.len()];
    let mut right_max_vec = vec![0; height.len()];
    for (i, v) in height.iter().enumerate() {
        left_max_vec[i] = left_max;
        if *v > left_max {
            left_max = *v;
        }
    }

    for (i, v) in height.iter().enumerate().rev() {
        right_max_vec[i] = right_max;
        if *v > right_max {
            right_max = *v;
        }
    }
    let mut res = 0;
    for i in 0..height.len() {
        let mut residual = min(right_max_vec[i], left_max_vec[i]) - height[i];
        residual = if residual >= 0 {
            residual
        } else {
            0
        };
        res += residual
    }

    res
}

#[cfg(test)]
mod test {
    use crate::other::algo_42::trap;

    #[test]
    fn test_trap() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        println!("trap={}", trap(height));
    }
}