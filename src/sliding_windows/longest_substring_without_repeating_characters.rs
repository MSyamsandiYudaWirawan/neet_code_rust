use std::collections::{HashSet};
use std::cmp::max;


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res: i32 = 0;
    let mut set:HashSet<u8> = HashSet::new();
    let mut l:usize = 0;
    let bytes:&[u8] = s.as_bytes();

    for r in 0..bytes.len() {
        while set.contains(&bytes[r]) {
            set.remove(&bytes[l]);
            l+=1;
        }
        set.insert(bytes[r]);
        res = max(res,(r-l+1) as i32);
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
