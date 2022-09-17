use std::cmp::min;

pub fn fast_add(value: i64, mut multiplier: i64) -> i64 {
    let mut res = 0;
    let mut contributor = value;

    while multiplier > 0 {
        if multiplier & 1 == 1 {
            res += contributor;
        }
        contributor += contributor;
        multiplier >>= 1;
    }
    res
}

pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
    let is_neg = if divisor < 0 { true } else { false } ^ if dividend < 0 {true} else {false};
    let mut dividend = dividend as i64;
    let mut divisor = divisor as i64;
    dividend = dividend.abs();
    divisor = divisor.abs();
    let mut left = 1 as i64;
    let mut right = dividend + 1;
    while left < right {
        let mut mid = (left + right) >> 1;
        let value= fast_add(mid, divisor) ;

        if value == dividend {
            break;
        }
        if value > dividend {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    let mut result = (left + right) >> 1;

    if fast_add(result, divisor) > dividend {
        result -= 1;
    }

    result = result * if is_neg {-1} else {1};
    result = min(result, 0x7fffffff);
    result as i32
}

#[cfg(test)]
mod test {
    use crate::lc::algo_29::{divide, fast_add};

    #[test]
    pub fn test() {
        println!("{}", fast_add(10, 3));
    }

    #[test]
    pub fn test_divide() {
        println!("{}", divide(-1010369383, -2147483648));
    }
}