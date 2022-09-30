use std::str::FromStr;

pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1_items = version1.split(".").collect::<Vec<&str>>();
    let v2_items = version2.split(".").collect::<Vec<&str>>();

    let mut i = 0;
    while i < v1_items.len() && i < v2_items.len() {
        let item1 = i32::from_str(v1_items[i]).unwrap();
        let item2 = i32::from_str(v2_items[i]).unwrap();
        if item1 > item2 {
            return 1;
        }else if item1 < item2 {
            return -1;
        }
        i += 1;
    }

    while i < v1_items.len() {
        let item = i32::from_str(v1_items[i]).unwrap();
        if item > 0 {
            return 1;
        }
        i += 1;
    }

    while i < v2_items.len() {
        let item = i32::from_str(v2_items[i]).unwrap();
        if item > 0 {
            return -1;
        }
        i += 1;
    }

    0
}

#[cfg(test)]
mod test {
    use crate::lc::algo_165::compare_version;

    #[test]
    pub fn test() {
        println!("{}", compare_version("1.0.00".to_string(), "1.0.00".to_string()));
    }
}