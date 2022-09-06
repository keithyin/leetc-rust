use std::cmp::max;

pub fn add_binary(a: String, b: String) -> String {
    let mut res = vec![0; max(a.len(), b.len())];
    let zero = '0' as u8;
    let a = a.as_str().as_bytes();
    let b = b.as_str().as_bytes();
    let mut carry = 0;
    let mut cur_pos = 0;
    let a_len = a.len() as i32 - 1;
    let b_len = b.len() as i32 - 1;
    let res_len = max(a_len, b_len);
    while (a_len - cur_pos) >= 0 && (b_len - cur_pos) >= 0 {
        let a_pos = (a_len - cur_pos) as usize;
        let b_pos = (b_len - cur_pos) as usize;
        let res_pos = (res_len - cur_pos) as usize;

        let mut cur_res = a[a_pos] - zero + b[b_pos] - zero + carry as u8;
        carry = cur_res / 2;
        cur_res = cur_res % 2;
        res[res_pos] = cur_res;
        cur_pos += 1;
    }

    while (a_len - cur_pos) >= 0 {
        let a_pos = (a_len - cur_pos) as usize;
        let res_pos = (res_len - cur_pos) as usize;

        let mut cur_res = a[a_pos] - zero+ carry as u8;
        carry = cur_res / 2;
        cur_res = cur_res % 2;
        res[res_pos] = cur_res;
        cur_pos += 1;
    }

    while (b_len - cur_pos) >= 0 {
        let b_pos = (b_len - cur_pos) as usize;
        let res_pos = (res_len - cur_pos) as usize;

        let mut cur_res = b[b_pos] - zero + carry as u8;
        carry = cur_res / 2;
        cur_res = cur_res % 2;
        res[res_pos] = cur_res;
        cur_pos += 1;
    }

    if carry > 0 {
        res.insert(0, 1);
    }

    res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}