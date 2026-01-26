use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut res: i32 = 0;
    let bytes: &[u8] = s.as_bytes();
    let mut max_count: i32 = 0;
    let mut l: usize = 0;
    let mut map: HashMap<u8, i32> = HashMap::new();

    for r in 0..bytes.len() {
        let count: &mut i32 = map.entry(bytes[r]).or_insert(0);
        *count += 1;
        max_count = cmp::max(max_count, *count);
        while (r - l + 1) as i32 > k + max_count {
            if let Entry::Occupied(mut entry) = map.entry(bytes[l]) {
                if entry.get() == &1 {
                    entry.remove();
                } else {
                    *entry.get_mut() -= 1;
                }
            }
            l += 1;
        }
        res = cmp::max(res, (r - l + 1) as i32);
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
