pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let r = s.clone().chars().rev().collect::<String>();
    
    s == r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(is_palindrome(121));
    }
    
    #[test]
    fn example_two() {
        assert!(!is_palindrome(-121));
    }
    
    #[test]
    fn example_three() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn example_four() {
        assert!(!is_palindrome(-101));
    }
}