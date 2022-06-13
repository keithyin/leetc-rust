pub fn is_power_of_three(mut n: i32) -> bool {


    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    while n % 3 == 0 {
        n = n / 3;
        if n == 1 {
            return true;
        }
    }
    false
}