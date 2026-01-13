use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let mut map_need:HashMap<char,i32> = HashMap::new();
    let mut map_window:HashMap<char,i32> = HashMap::new();

    for c in t.chars() {
        *map_need.entry(c).or_insert(0) += 1;
    }
    let mut have:usize = 0;
    let  need:usize = map_need.len();
    let bytes_s:&[u8] = s.as_bytes();
    // let mut bytes_t:&[u8] = t.as_bytes();

    let mut l:usize = 0;
    let mut res:String = String::new();
    let mut res_len:usize = usize::MAX;
    for r in 0..s.len() {
        let c_r = bytes_s[r] as char;
        if let Some(need_count) = map_need.get(&c_r){
            let have_count = map_window.entry(c_r).or_insert(0);
            *have_count +=1;

            if need_count == have_count {
                have +=1;
            }
        }
        while have == need {
            let c_l = bytes_s[l] as char;
            if res_len > (r-l+1) {
                res = s[l..r+1].to_string();
                res_len = r-l+1;
            }
            if let Some(need_count) = map_need.get(&c_l){
                let have_count = map_window.entry(c_l).or_insert(0);
                *have_count -=1;

                if need_count > have_count {
                    have -=1;
                }
            }
            l+=1;
        }
    }

    return res;
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
        assert_eq!(
            min_window("a".to_string(), "a".to_string()),
            "a"
        );
        assert_eq!(
            min_window("a".to_string(), "aa".to_string()),
            ""
        );
    }
}
