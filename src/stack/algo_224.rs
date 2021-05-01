
pub fn basic_caculator(mut expression: String) {
    expression = expression.replace(" ", "");
    let tokens: Vec<&str> = expression.split("(|)|+|-|*|/").collect();
    println!("{:?}", tokens);
}

#[cfg(test)]
mod test {
    use crate::stack::algo_224::basic_caculator;

    #[test]
    fn test() {
        basic_caculator(String::from("1+2*(1-2)/10"))
    }
}