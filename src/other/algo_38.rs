pub fn count_and_say(n: i32) -> String {
    let mut res_str = "1".to_string();

    for i in 2..n+1 {
        let mut items = vec![];
        let mut counts = vec![];
        let mut prev_c = '0';
        let mut prev_count = 0;
        for cur_c in res_str.chars() {
            if prev_c != cur_c && prev_c != '0' {
                items.push(prev_c);
                counts.push(prev_count);
                prev_count = 0;
            }
            prev_c = cur_c;
            prev_count += 1
        }
        items.push(prev_c);
        counts.push(prev_count);

        res_str = counts.iter().zip(items.iter()).map(|(a, b)| format!("{}{}", a, b)).collect();
    }
    res_str
}

#[cfg(test)]
mod test {
    use crate::other::algo_38::count_and_say;

    #[test]
    fn test_count_and_say() {
        println!("{}", count_and_say(3));
    }
}