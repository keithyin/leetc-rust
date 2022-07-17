use std::cmp::min;

pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for row in 1..triangle.len() {
        for col in 0..row+1 {
            let mut min_val = i32::MAX;
            for upper_idx in [col as i32 - 1, col as i32] {
                min_val = if upper_idx>=0 && upper_idx < row as i32 &&(triangle[row-1][upper_idx as usize] + triangle[row][col]) < min_val {
                    triangle[row-1][upper_idx as usize] + triangle[row][col]
                } else {
                    min_val
                };
            }

            triangle[row][col] = min_val;
        }
    }
    let mut min_val = i32::MAX;
    for v in triangle[triangle.len()-1].iter() {
        min_val = min(min_val, *v);
    }
    min_val
}

#[cfg(test)]
mod test {
    use crate::lc::algo_120::minimum_total;

    #[test]
    pub fn test_t() {
        let inp = vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]];
        println!("{}", minimum_total(inp));
    }
}