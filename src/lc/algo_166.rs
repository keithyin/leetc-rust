use std::collections::HashMap;

pub fn fraction_to_decimal(mut numerator: i32, mut denominator: i32) -> String {
    let mut res = vec![];
    let mut v_2_idx = HashMap::new();
    let mut numerator = numerator as i64;
    let mut denominator = denominator as i64;
    let is_neg = if numerator * denominator < 0 {
        true
    } else {
        false
    };

    numerator = numerator.abs();
    denominator = denominator.abs();

    let loop_idx = loop {

        if v_2_idx.contains_key(&numerator) {
            if v_2_idx[&numerator] == 0 {
                res.push(res[0]);
            }
            break v_2_idx[&numerator] as i32;
        }

        res.push(numerator / denominator);

        v_2_idx.insert(numerator, res.len()-1);

        numerator %= denominator;
        if numerator == 0 {
            break -1;
        }
        numerator *= 10;
    };

    let mut res_str = if is_neg {
        "-".to_string()
    } else {
        "".to_string()
    };
    for i in 0..res.len() {
        if i == 0 {
            res_str += format!("{}", res[i]).as_str();
            if res.len() > 1 {
                res_str += ".";
            }
            if loop_idx == 0 {
                res_str += "(";
            }
            continue;
        }

        if loop_idx == i as i32 {
            res_str += "("
        }
        res_str += res[i].to_string().as_str();
    }
    if loop_idx >= 0 {
        res_str += ")"
    }

    res_str
}

#[cfg(test)]
mod test {
    use crate::lc::algo_166::fraction_to_decimal;

    #[test]
    fn test_fraction_to_decimal() {
        println!("{}", fraction_to_decimal(0, 3));
    }
}