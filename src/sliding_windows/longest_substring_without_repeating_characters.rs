use std::cmp;
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res:i32 = 0;
    let mut l:usize = 0;
    let chars:Vec<char> = s.chars().collect();
    let mut set:HashSet<char> = HashSet::new();

    for r in 0..chars.len(){
        while set.contains(&chars[r]){
            set.remove(&chars[l]);
            l+=1;
        }
        set.insert(chars[r]);
        res = cmp::max(res,(r-l+1) as i32);
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
