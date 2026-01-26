use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
   if s.len() != t.len() {
       return false;
   }
    let s_bytes: &[u8] = s.as_bytes();
    let t_bytes: &[u8] = t.as_bytes();
    let mut s_map:HashMap<u8,i32> = HashMap::new();
    let mut t_map:HashMap<u8,i32> = HashMap::new();
    for i in 0..s_bytes.len() {
        *s_map.entry(s_bytes[i]).or_insert(0) +=1;
        *t_map.entry(t_bytes[i]).or_insert(0) +=1;
    }
    return s_map == t_map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert_eq!(valid_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(valid_anagram("rat".to_string(), "car".to_string()), false);
    }
}

