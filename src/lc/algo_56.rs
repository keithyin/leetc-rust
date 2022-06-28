
pub fn merge_span(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut res = vec![];

    let mut prev = intervals[0].clone();

    for span in intervals.iter().skip(1) {
        if span[0] <= prev[1] {
            prev[1] = if prev[1] >= span[1] {
                prev[1]
            } else {
                span[1]
            };
        } else {
            res.push(prev);
            prev = span.clone();
        }
    }
    res.push(prev);
    res
}

#[cfg(test)]
mod test {
    use crate::lc::algo_56::merge_span;

    #[test]
    fn test_merge_span() {
        let spans = vec![vec![1, 2], vec![2, 3], vec![1, 4]];
        println!("{:?}", merge_span(spans));

    }
}
