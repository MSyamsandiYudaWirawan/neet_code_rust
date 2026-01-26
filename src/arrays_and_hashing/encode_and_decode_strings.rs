pub fn encode(strs: Vec<String>) -> String {
    let mut res: String = String::new();
    for str in strs {
        res.push_str(&str.len().to_string());
        res.push_str("#");
        res.push_str(&str);
    }
    return res;
}

pub fn decode(str: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let bytes: &[u8] = str.as_bytes();
    while i < str.len(){
        let mut j:usize = i;
        while bytes[j] != b'#'{
            j+=1;
        }
        let len:usize = str::from_utf8(&bytes[i..j]).unwrap().parse().unwrap();
        i=j+1;
        let s:&str = str::from_utf8(&bytes[i..i+len]).unwrap();
        i=i+len;
        res.push(s.to_string());
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
