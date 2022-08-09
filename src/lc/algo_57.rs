
pub fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {

    let mut begin = 0;
    let mut end = arr.len() as i32;

    while begin < end {
        let mid = (begin + end) / 2;
        if arr[mid as usize] == target {
            break;
        } else if arr[mid as usize] > target {
            end = mid;
        } else {
            begin = mid + 1;
        }
    }

    let mut pos = (begin + end) / 2;
    if (pos as usize) < arr.len() && arr[pos as usize] < target {
        pos += 1;
    }

    pos
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals.iter().flatten().map(|x| *x).collect::<Vec<i32>>();
    let mut left_pos = binary_search(&intervals, new_interval[0]);
    let mut right_pos = binary_search(&intervals, new_interval[1]);

    if (right_pos as usize) < intervals.len() && intervals[right_pos as usize] == new_interval[1] {
        right_pos += 1;
    }

    intervals.insert(right_pos as usize, new_interval[1]);
    intervals.insert(left_pos as usize, new_interval[0]);
    right_pos += 1;
    let delete_begin = if left_pos % 2 == 0 {
        left_pos + 1
    } else {
        left_pos
    };

    let delete_end = if right_pos % 2 == 0 {
        right_pos
    } else {
        right_pos - 1
    };

    for _ in 0..(delete_end - delete_begin + 1) {
        intervals.remove(delete_begin as usize);
    }

    let mut result = vec![];
    let mut tmp = vec![];
    for i in 0..intervals.len() {
        tmp.push(intervals[i]);
        if (i+1) % 2 == 0 {
            result.push(tmp);
            tmp = vec![];
        }
    }

    result

}

#[cfg(test)]
mod test {
    use crate::lc::algo_57::insert;

    #[test]
    pub fn test() {
        println!("{:?}", insert(vec![vec![1, 3], vec![4, 5]], vec![2, 9]));
    }

}