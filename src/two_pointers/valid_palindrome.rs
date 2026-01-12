pub fn is_palindrome(s: String) -> bool {
    let char:Vec<char> = s.chars().collect();
    let mut l:usize = 0;
    let mut r:usize = char.len() -1;

    while l<r {
        while l<r && !is_alnum(char[l]){
            l+=1;
        }
        while l<r && !is_alnum(char[r]) {
            r-=1;
        }
        if l<r && char[l].to_ascii_lowercase() != char[r].to_ascii_lowercase(){
            return false;
        }
        l+=1;
        r-=1;
    }
    return true;
}

fn is_alnum(c:char) -> bool{
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9');
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
