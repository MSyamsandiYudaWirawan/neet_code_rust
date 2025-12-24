use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len(){
        return false;
    }
    let mut ms:HashMap<char,i32> = HashMap::new();
    let mut mt:HashMap<char,i32> = HashMap::new();
    for (ct,cs) in s.chars().zip(t.chars()){
        *ms.entry(cs).or_insert(0)+=1;
        *mt.entry(ct).or_insert(0)+=1;
    }
    return ms==mt;
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

