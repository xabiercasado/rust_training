use std::collections::HashMap;

static mut ROMAN: Option<String> = None;
static mut RESULT: Option<i32> = None;

unsafe fn input(roman: String) {
    ROMAN = Some(roman);
}

unsafe fn algorithm() {
    let roman = ROMAN.clone().unwrap();
    
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

    RESULT = Some(number);

    return;
}

unsafe fn output() -> i32 {
    RESULT.clone().unwrap()
}

pub unsafe fn roman_to_int(roman: String) -> i32 {
    input(roman);

    algorithm();

    output()
}

#[test]
fn test() {
    assert_eq!(3, unsafe { roman_to_int(String::from("III")) });
    assert_eq!(4, unsafe { roman_to_int(String::from("IV")) });
    assert_eq!(9, unsafe { roman_to_int(String::from("IX")) });
    assert_eq!(58, unsafe { roman_to_int(String::from("LVIII")) });
    assert_eq!(1994, unsafe { roman_to_int(String::from("MCMXCIV")) });
}