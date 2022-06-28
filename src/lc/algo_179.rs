
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut str_nums: Vec<String> = nums.iter().map(|v|  v.to_string()).collect();
    str_nums.sort_by(|a, b| (a.to_owned() + b).partial_cmp(&(b.to_owned() + a)).unwrap().reverse());
    let mut res = str_nums.join("");
    return if res.chars().nth(0).unwrap() == '0' {
        "0".to_string()
    } else {
        res
    };
}

#[cfg(test)]
mod test {
    use crate::lc::algo_179::largest_number;

    #[test]
    fn test_rotate() {
        let mut nums = vec![0, 1, 2, 3];
        println!("{}", largest_number(nums));
    }
}