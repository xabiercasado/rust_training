use std::collections::HashMap;

pub fn roman_to_int(roman: String) -> i32 {
    let dict: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    
    let mut number = 0;
    let mut prev = 0;

    for r in roman.chars() {
        if let Some(&c) = dict.get(&r) {
            number += c;

            if c > prev {
                number -=  prev + prev;
            }

            prev = c;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let s = String::from("III");

        assert_eq!(3, roman_to_int(s));
    }

    #[test]
    fn example_two() {
        let s = String::from("IV");
        
        assert_eq!(4, roman_to_int(s));
    }

    #[test]
    fn example_three() {
        let s = String::from("IX");

        assert_eq!(9, roman_to_int(s));
    }

    #[test]
    fn example_four() {
        let s = String::from("LVIII");

        assert_eq!(58, roman_to_int(s));
    }

    #[test]
    fn example_five() {
        let s = String::from("MCMXCIV");

        assert_eq!(1994, roman_to_int(s));
    }
}
