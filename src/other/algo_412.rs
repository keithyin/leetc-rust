pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec![];
    for i in 1..n+1 {
        let mut tmp_s = "".to_string();
        let mut flag = false;
        if i % 3 == 0 {
            tmp_s.push_str("Fizz");
            flag = true;
        }
        if i % 5 == 0 {
            tmp_s.push_str("Buzz");
            flag = true;
        }
        if !flag {
            tmp_s.push_str(format!("{}", i).as_str());
        }
        result.push(tmp_s);
    }
    result
}

#[cfg(test)]
mod test {

    #[test]
    fn test_fizz_buzz() {

    }
}