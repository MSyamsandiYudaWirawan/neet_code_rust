pub fn encode(strs: Vec<String>) -> String {
    let mut string: String = String::new();

    for str in strs {
        string.push_str(&str.len().to_string());
        string.push_str("#");
        string.push_str(&str);
    }

    return string;
}

pub fn decode(str: String) -> Vec<String> {
    let mut i: usize = 0;
    let mut res: Vec<String> = Vec::new();
    let char: Vec<char> = str.chars().collect();
    while i < char.len() {
        let mut j: usize = i;
        while char[j] != '#' {
            j = j + 1;
        }
        let length_str: String = char[i..j].iter().collect();
        let length: usize = length_str.parse::<usize>().unwrap();
        i = j + 1;
        let s: String = char[i..i + length].iter().collect();
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
