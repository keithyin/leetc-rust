
pub fn space_gen(n: usize) -> String {
    let mut result = "".to_string();
    for i in 0..n {
        result.push_str(" ");
    }
    result
}

pub fn reorder_spaces(text: String) -> String {
    let text = text.as_str();
    let mut words = vec![];
    let mut num_spaces = 0;

    let mut pre_space_idx = -1;
    for i in 0..text.len() {
        if &text[i..i+1] == " " {
            num_spaces += 1;
            if (i as i32 - pre_space_idx) > 1 {
                words.push(&text[(pre_space_idx + 1) as usize..i])
            }
            pre_space_idx = i as i32;
        }
    }
    let i = text.len();
    if(i as i32 - pre_space_idx) > 1 {
        words.push(&text[(pre_space_idx+1) as usize .. i])
    }

    let num_word = words.len();
    let num_interval = num_word - 1;

    let interval_spaces = if num_interval == 0 {
        0
    } else {
        num_spaces / num_interval
    };

    let tail_spaces = num_spaces - num_interval * interval_spaces;
    println!("{} {} {} {}", num_spaces, num_interval, interval_spaces, tail_spaces);

    let interval_space = space_gen(interval_spaces);
    let interval_space = interval_space.as_str();
    let tail_space = space_gen(tail_spaces);
    let tail_space = tail_space.as_str();

    let mut result = "".to_string();
    for i in 0..words.len() {
        result.push_str(words[i]);
        if i < (words.len()-1) {
            result.push_str(interval_space);
        }
    }
    result.push_str(tail_space);
    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_1592::reorder_spaces;

    #[test]
    fn test() {
        println!("|{}|", reorder_spaces("  this   is  a sentence ".to_string()));
    }
}