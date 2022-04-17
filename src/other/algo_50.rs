
pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut n = n;
    let is_n_negative = n < 0;
    n = if n < 0 {
        -n
    } else {
        n
    };

    let res = my_pow_core(x, n);
    return if is_n_negative {
        1.0 / res
    } else {
        res
    };

}

pub fn my_pow_core(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let y = my_pow_core(x, n >> 1);
    return if n & 1 == 1 {
        y * y * x
    } else {
        y * y
    };
}

pub fn my_pow_core_2(x: f64, n: i32) -> f64 {
    let mut n = n;
    let mut ans = 1.0;
    let mut x_contribute = x;
    while n > 0 {
        if n & 1 == 1 {
            ans *= x;
        }
        x_contribute *= x_contribute;
        n >>= 1;
    }
    ans
}


#[cfg(test)]
mod test {

    #[test]
    fn test_my_pow() {
        let a = 10;
        println!("{}", a << 1);
    }
}

