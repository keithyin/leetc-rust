use std::collections::HashSet;

fn compute_next(mut n: i32) -> i32 {
    let mut result = 0;
    while n > 0 {
        result += (n % 10) * (n % 10);
        n /= 10;
    }

    return result;
}

pub fn is_happy(n: i32) -> bool {
    let mut visited = HashSet::new();

    let mut next = n;
    loop {
        if next == 1 {
            return true;
        }
        if visited.contains(&next) {
            return false;
        }

        visited.insert(next);
        next = compute_next(next);
    }


}

