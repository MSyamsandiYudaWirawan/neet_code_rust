use std::collections::HashMap;

pub fn group_anagrams(strs:Vec<String>) -> Vec<Vec<String>>{
    let mut map:HashMap<String,Vec<String>> =  HashMap::new();

    for str in strs{
        let mut chars:Vec<char>= str.chars().collect();
        chars.sort();
        let sorted_string = chars.into_iter().collect();

        map.entry(sorted_string).or_insert(Vec::new()).push(str);
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
