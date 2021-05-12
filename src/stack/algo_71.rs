
pub fn simplify_path(path: String) -> String {
    let mut path_stack = vec![];
    for item in path.split("/") {
        match item {
            "/" | "" | "."=> {},
            ".." => {
                path_stack.pop();
            },
            _ => {
                path_stack.push(String::from(item));
            }
        }
    }
    return String::from("/") + &path_stack.join("/");
}

#[cfg(test)]
mod test {
    use crate::stack::algo_71::simplify_path;

    #[test]
    fn test_simplify_path() {
        let path = String::from("/a/./b/../../c/");
        let res = simplify_path(path);
        println!("{}", res);
        assert_eq!(res, String::from("/c"))
    }
}