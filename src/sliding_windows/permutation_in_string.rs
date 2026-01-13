use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let chars_s1:Vec<char> = s1.chars().collect();
    let chars_s2:Vec<char> = s2.chars().collect();
    let mut map_s1:HashMap<char,i32> = HashMap::new();
    let mut map_s2:HashMap<char,i32> = HashMap::new();

    let mut l:usize = 0;

    for i in 0..chars_s1.len() {
        *map_s1.entry(chars_s1[i]).or_insert(0) += 1;
    }
    for r in 0..chars_s2.len() {
        *map_s2.entry(chars_s2[r]).or_insert(0) += 1;
        if (r-l+1) > s1.len() {
            if let Entry::Occupied(mut entry) = map_s2.entry(chars_s2[l]){
                if *entry.get() == 1 {
                    entry.remove();
                }
                else {
                    *entry.get_mut() -= 1;
                }
                l+=1;
            }
        }

        if (r-l+1) == s1.len() && map_s1 == map_s2 {
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
        assert_eq!(check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
        assert_eq!(check_inclusion("ab".to_string(), "eidboaoo".to_string()), false);
    }
}
