use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {

    let mut s1map = HashMap::new();
    let mut s2map = HashMap::new();

    for byte in s.as_bytes() {
        if s1map.contains_key(byte) {
            s1map.insert(*byte, s1map[byte] + 1);
        } else {
            s1map.insert(*byte, 1);
        }
    }

    for byte in t.as_bytes() {
        if s2map.contains_key(byte) {
            s2map.insert(*byte, s2map[byte] + 1);
        } else {
            s2map.insert(*byte, 1);
        }
    }

    for (k, v ) in &s1map {
        if !s2map.contains_key(k) || s2map[k] != *v {
            return false;
        }
    }
    true
}