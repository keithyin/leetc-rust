pub fn my_sqrt(x: i32) -> i32 {

    let mut begin = 1;
    let mut end = x/2;

    while begin <= end {
        let mut cur = (begin + end) / 2;
        if cur * cur == x {
            return cur
        }else if cur * cur < x {
            begin = cur + 1;
        } else {
            end = cur - 1;
        }
    }

    return if begin * begin < x {
        begin
    }else {
        end
    }
}

pub fn my_sqrt_2(x: i32) -> i32{
    0
}

#[cfg(test)]
mod test {

    #[test]
    fn test_my_sqrt() {

    }
}