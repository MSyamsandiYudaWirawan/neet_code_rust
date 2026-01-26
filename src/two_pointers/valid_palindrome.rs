

pub fn is_palindrome(s: String) -> bool {
    let mut l:usize =0;
    let mut r:usize = s.len()-1;
    let bytes:&[u8] = s.as_bytes();

    while l<r {
        while l<r && !is_alnum(bytes[l] as char) {
            l+=1;
        }
        while l<r && !is_alnum(bytes[r] as char){
            r-=1;
        }
        if l<r && bytes[l].to_ascii_lowercase() != bytes[r].to_ascii_lowercase() {
            return false;
        }
        l+=1;
        r-=1;
    }

    return true;
}

fn is_alnum(c:char) -> bool{
    return (c>='a' && c<='z') || (c>='A' && c<='Z') || (c>='0' && c<='9');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(is_palindrome("race a car".to_string()), false);
        assert_eq!(is_palindrome(" ".to_string()), true);
    }
}
