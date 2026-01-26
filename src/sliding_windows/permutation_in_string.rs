use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let s1_bytes: &[u8] = s1.as_bytes();
    let s2_bytes: &[u8] = s2.as_bytes();
    let mut s1_map: HashMap<u8, i32> = HashMap::new();
    let mut s2_map: HashMap<u8, i32> = HashMap::new();
    for i in 0..s1_bytes.len() {
        *s1_map.entry(s1_bytes[i]).or_insert(0) += 1;
    }

    let mut l: usize = 0;
    for r in 0..s2_bytes.len() {
        *s2_map.entry(s2_bytes[r]).or_insert(0) += 1;

        while (r - l + 1) > s1.len() {
            if let Entry::Occupied(mut entry) = s2_map.entry(s2_bytes[l]) {
                if entry.get() == &1 {
                    entry.remove();
                } else {
                    *entry.get_mut() -= 1;
                }
            }
            l+=1;
        }
        if (r-l+1) == s1.len() && s1_map == s2_map {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }
}
