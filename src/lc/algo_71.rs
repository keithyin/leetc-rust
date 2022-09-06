
pub fn simplify_path(path: String) -> String {
    let items = path.split("/").collect::<Vec<&str>>();
    let mut res = vec![];
    for token in items.iter() {
        let token = *token;
        if token == "" || token == "." {
            continue;
        }
        if token == ".." {
            res.pop();
        } else {
            res.push(token);
        }
    }
    let mut res_str = "/".to_string();
    res_str.push_str(res.join("/").as_str());
    res_str

}

#[cfg(test)]
mod test {
    use crate::lc::algo_71::simplify_path;

    #[test]
    pub fn test() {
        println!("{}", simplify_path("/../".to_string()));
    }
}