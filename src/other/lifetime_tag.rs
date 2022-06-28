


fn demo1<'a, 'b: 'a>(p1: &'a str, p2: &'b str) -> &'b str {
    p2
}


#[cfg(test)]
mod test {
    use crate::other::lifetime_tag::demo1;

    #[test]
    fn testDemo1() {
        let p1 =  String::from("world");
        let mut c;
        {
            let p2 = String::from("hello");

            c = demo1(p2.as_str(), p1.as_str());
            // println!("{}", c);
        }
        println!("{}", c);
    }
}