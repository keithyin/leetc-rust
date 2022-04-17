pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    if matrix.len() == 0 {
        return;
    }
    let num_row = matrix.len();
    let num_col = matrix[0].len();

    // 第一行是否有0，第一列是否有0用额外的变量表示
    let mut first_row_has_zero = false;
    for v in matrix[0].iter() {
        if *v == 0 {
            first_row_has_zero = true;
            break;
        }
    }

    let mut first_col_has_zero = false;
    for row in 0..num_row {
        if matrix[row][0] == 0 {
            first_col_has_zero = true;
            break;
        }
    }

    // 第一行表示，以下的每列是否有 0
    // 第一列表示，一下的每行是否有 0
    for row in 1..num_row {
        for col in 1..num_col {
            if matrix[row][col] == 0 {
                matrix[0][col] = 0;
                matrix[row][0] = 0;
            }
        }
    }

    for row in 0..num_row {
        if matrix[row][0] == 0 {
            for inner_col in 0..num_col {
                matrix[row][inner_col] = 0;
            }
        }
    }

    for col in 0..num_col {
        if matrix[0][col] == 0 {
            for inner_row in 0..num_row {
                matrix[inner_row][col] = 0;
            }
        }
    }

    if first_col_has_zero {
        for inner_row in 0..num_row {
            matrix[inner_row][0] = 0;
        }
    }

    if first_row_has_zero {
        for inner_col in 0..num_col {
            matrix[0][inner_col] = 0;
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_set_zeroes() {
        for i in 0..1 + 2 {
            println!("{}", i);
        }
    }
}