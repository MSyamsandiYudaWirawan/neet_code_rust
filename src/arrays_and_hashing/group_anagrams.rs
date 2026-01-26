use std::collections::HashMap;

pub fn group_anagrams(strs:Vec<String>) -> Vec<Vec<String>>{
    let mut map:HashMap<String,Vec<String>> = HashMap::new();
    for str in strs {
        let mut carr:Vec<char> = str.chars().collect();
        carr.sort();
        let sorted_str:String = carr.iter().collect();
        map.entry(sorted_str).or_insert(Vec::new()).push(str);
    }
    return map.into_values().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let mut result = group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        
        let mut expected = vec![vec!["bat".to_string()], vec!["ate".to_string(), "eat".to_string(), "tea".to_string()], vec!["nat".to_string(), "tan".to_string()]];
        for group in &mut expected {
            group.sort();
        }
        expected.sort();
        
        assert_eq!(result, expected);
        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec!["".to_string()]]);
    }
}
