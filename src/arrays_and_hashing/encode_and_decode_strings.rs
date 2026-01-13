pub fn encode(strs: Vec<String>) -> String {
    let mut res:String = String::new();
    for str in strs {
        res.push_str(&str.len().to_string());
        res.push_str("#");
        res.push_str(&str);
    }
    return res;
}

pub fn decode(str: String) -> Vec<String> {
    let mut res:Vec<String> = Vec::new();
    let chars:Vec<char> = str.chars().collect();
    let mut i:usize = 0;

    while i<chars.len() {
        let mut j:usize = i;
        while chars[j] != '#' {
            j +=1;
        }
        let length_str:String = chars[i..j].iter().collect();
        i = j + 1;
        let length:usize = length_str.parse::<usize>().unwrap();
        let s:String = chars[i..i+length].iter().collect();
        i = i + length;
        res.push(s);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let strs = vec![String::from("hello"), String::from("world")];
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(strs, decoded);
    }

    #[test]
    fn test_empty_strings() {
        let strs = vec![String::from(""), String::from("a"), String::from("")];
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(strs, decoded);
    }
}
