use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return "".to_string();
    }
    let mut map_need: HashMap<u8, i32> = HashMap::new();
    let mut map_have: HashMap<u8, i32> = HashMap::new();
    let bytes_s: &[u8] = s.as_bytes();
    let bytes_t: &[u8] = t.as_bytes();
    for i in 0..bytes_t.len() {
        *map_need.entry(bytes_t[i]).or_insert(0) += 1;
    }
    let mut have: i32 = 0;
    let need: i32 = map_need.len() as i32;
    let mut l: usize = 0;
    let mut res_len: i32 = i32::MAX;
    let mut res_l: usize = 0;
    let mut res_r: usize = 0;
    for r in 0..bytes_s.len() {
        if let Some(need_count) = map_need.get(&bytes_s[r]) {
            let have_count: &mut i32 = map_have.entry(bytes_s[r]).or_insert(0);
            *have_count += 1;
            if need_count == have_count {
                have += 1;
            }
        }
        while have == need {
            if res_len > (r - l + 1) as i32 {
                res_len = (r - l + 1) as i32;
                res_l = l;
                res_r = r;
            }
            if let Some(need_count) = map_need.get(&bytes_s[l]){
                let have_count: &mut i32 = map_have.entry(bytes_s[l]).or_default();
                *have_count -= 1;
                if *have_count < *need_count {
                    have -=1;
                }
            }
            l+=1;
        }
    }
    return s[res_l..res_r + 1].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
    }
}
