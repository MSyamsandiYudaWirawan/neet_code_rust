use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut maps:HashMap<char,i32> = HashMap::new();
    let mut mapt:HashMap<char,i32> = HashMap::new();

    for (cs,ct) in s.chars().zip(t.chars()) {
        *maps.entry(cs).or_insert(0) += 1;
        *mapt.entry(ct).or_insert(0) += 1;
    }
    return maps == mapt;
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

