use std::cmp::min;

pub fn trailing_zeroes(n: i32) -> i32 {
    let mut count2 = 0;
    let mut count5 = 0;
    for i in 2..(n+1) {
        let mut count_for2_i = i;
        while count_for2_i > 0 {
            if count_for2_i % 2 == 0 {
                count2 += 1;
                count_for2_i /= 2;
            } else {
                break;
            }
        }

        let mut count_for5_i = i;
        while count_for5_i > 0 {
            if count_for5_i % 5 == 0 {
                count5 += 1;
                count_for5_i /= 5;
            } else {
                break;
            }
        }
    }
    min(count2, count5)
}