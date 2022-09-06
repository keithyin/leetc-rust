
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry= 1;
    for i in (0..digits.len()).rev() {
        let mut cur = digits[i] + carry;
        carry = cur / 10;
        cur = cur % 10;
        digits[i] = cur;
        if carry == 0 {
            break;
        }
    }
    if carry > 0 {
        digits.insert(0, carry);
    }
    digits
}