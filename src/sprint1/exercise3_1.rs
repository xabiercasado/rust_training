pub fn is_valid(str: String) -> bool {
    let mut ordered: Vec<char> = vec![];

    for sign in str.chars() {
        match sign {
            '(' | '[' | '{' => ordered.push(sign),
            ')' | ']' | '}' => if let Some(previous_sign) = ordered.pop() {
                if !((previous_sign == '(' && sign == ')')
                    || (previous_sign == '[' && sign == ']') 
                    || (previous_sign == '{' && sign == '}')) { return false; }
            } else {
                return false;
            },
            _ => (),
        }
    }
    
    ordered.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let str = String::from("()");

        assert!(is_valid(str));
    }

    #[test]
    fn example_two() {
        let str =  String::from("()[]{}");

        assert!(is_valid(str));
    }

    #[test]
    fn example_three() {
        let str =  String::from("(]");

        assert!(!is_valid(str));
    }

    #[test]
    fn example_four() {
        let str =  String::from("([)]");

        assert!(!is_valid(str));
    }

    #[test]
    fn example_five() {
        let str =  String::from("{[]}");

        assert!(is_valid(str));
    }
}