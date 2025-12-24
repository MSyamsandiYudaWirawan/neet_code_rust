
pub fn encode(strs:Vec<String>) -> String{
    let mut result = String::new();
    for s in strs {
        result.push_str(&s.len().to_string());
        result.push_str("#");
        result.push_str(&s);
    }
    return result;
}

pub fn decode(str:String) -> Vec<String>{

    let mut i:usize = 0;
    let byte:Vec<u8> = str.as_bytes().to_vec();
    let mut result:Vec<String> = Vec::new();

    while i < str.len() {
        let mut j:usize = i;
        while byte[j] != b'#' {
            j+=1;
        }
        let len:i32 = String::from_utf8(byte[i..j].to_vec()).unwrap().parse().unwrap();
        i=j+1;
        let s = String::from_utf8(byte[i..i+len as usize].to_vec()).unwrap();
        result.push(s);
        i=len as usize + i;
    }
    return result;
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