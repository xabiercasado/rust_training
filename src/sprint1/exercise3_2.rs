static mut STR: Option<String> = None;
static mut RESULT: Option<bool> = None;

unsafe fn input(str: String) {
    STR = Some(str);
}

unsafe fn algorithm() {
    let str = STR.clone().unwrap();    
    let mut ordered: Vec<char> = vec![];

    for sign in str.chars() {
        match sign {
            '(' | '[' | '{' => ordered.push(sign),
            ')' | ']' | '}' => if let Some(previous_sign) = ordered.pop() {
                if !((previous_sign == '(' && sign == ')')
                    || (previous_sign == '[' && sign == ']') 
                    || (previous_sign == '{' && sign == '}')) {
                        RESULT = Some(false);

                        return;
                    }
            } else {
                RESULT = Some(false);

                return;
            },
            _ => (),
        }
    }
    RESULT = Some(ordered.is_empty());
    
    return;
}

unsafe fn output() -> bool {
    RESULT.clone().unwrap()
}

pub unsafe fn is_valid(str: String) -> bool {
    input(str);

    algorithm();

    output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let str = String::from("()");

        assert!(unsafe { is_valid(str) });
    }

    #[test]
    fn example_two() {
        let str =  String::from("()[]{}");

        assert!(unsafe { is_valid(str) });
    }

    #[test]
    fn example_three() {
        let str =  String::from("(]");

        assert!(unsafe { !is_valid(str) });
    }

    #[test]
    fn example_four() {
        let str =  String::from("([)]");

        assert!(unsafe { !is_valid(str) });
    }

    #[test]
    fn example_five() {
        let str =  String::from("{[]}");

        assert!(unsafe { is_valid(str) });
    }
}