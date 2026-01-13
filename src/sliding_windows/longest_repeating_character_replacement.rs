use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut res:i32 = 0;
    let mut l:usize = 0;
    let chars:Vec<char> = s.chars().collect();
    let mut map:HashMap<char,i32> = HashMap::new();
    let mut max_count:i32 = 0;
    for r in 0..chars.len(){
        *map.entry(chars[r]).or_insert(0) += 1;
        if let Some(count) = map.get(&chars[r]){
            max_count = cmp::max(max_count,*count);
        }
        while (r - l + 1) as i32 > max_count + k {
            if let Entry::Occupied(mut e) = map.entry(chars[l]) {
                if *e.get() == 1 {
                    e.remove();
                } else {
                    *e.get_mut() -= 1;
                }
            }
            l += 1;
        }
        res = cmp::max(res,(r-l+1) as i32);

    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
