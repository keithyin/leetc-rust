use std::str::FromStr;

pub fn restore_ip_addresses_core(s: &str, cur_pos: usize, tracer: &mut Vec<Vec<String>>, result: &mut Vec<String>) {
    if tracer.len() > 4 && tracer[4].len() > 0{
        return;
    }
    if cur_pos >= s.len() {
        if tracer.last_mut().unwrap().len() == 0 {
            tracer.pop();
        }
        if tracer.len() != 4 {
            return;
        }

        let cur_seg = tracer.last_mut().unwrap();
        if i32::from_str(cur_seg.join("").as_str()).unwrap() > 255 {
            return;
        }

        if cur_seg.len() > 1 && cur_seg[0] == "0".to_string() {
            return;
        }

        result.push(
            tracer.iter()
                .map(|x|x.join(""))
                .collect::<Vec<String>>()
                .join("."));
        return;
    }

    if cur_pos == 0 {
        tracer.push(vec![]);
    }
    let mut cur_seg = tracer.last_mut().unwrap();
    if cur_seg.len() == 3 {
        if cur_seg[0] == "0".to_string() || i32::from_str(cur_seg.join("").as_str()).unwrap() > 255{
            return;
        }

        tracer.push(vec![]);
        cur_seg = tracer.last_mut().unwrap();
        cur_seg.push(s[cur_pos..cur_pos+1].to_string());

        restore_ip_addresses_core(s, cur_pos+1, tracer, result);
        tracer.pop();

    } else {

        if cur_seg.len() > 1 && cur_seg[0] == "0".to_string() {
            return;
        }

        cur_seg.push(s[cur_pos..cur_pos+1].to_string());
        restore_ip_addresses_core(s, cur_pos+1, tracer, result);
        tracer.last_mut().unwrap().pop();

        if tracer.last_mut().unwrap().len() == 0 {
            return;
        }

        tracer.push(vec![]);
        cur_seg = tracer.last_mut().unwrap();
        cur_seg.push(s[cur_pos..cur_pos+1].to_string());
        restore_ip_addresses_core(s, cur_pos+1, tracer, result);
        tracer.pop();
    }
}

pub fn restore_ip_addresses(s: String) -> Vec<String> {

    let mut result = vec![];
    let mut tracer = vec![];
    restore_ip_addresses_core(s.as_str(), 0, &mut tracer, &mut result);

    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_93::restore_ip_addresses;

    #[test]
    pub fn test () {
        let s = "00000".to_string();
        println!("{:?}", restore_ip_addresses(s));
    }
}