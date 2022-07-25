use leetcode_prelude::vec_string;

static mut STRS: Option<Vec<String>> = None;
static mut RESULT: Option<String> = None;

unsafe fn input(strs: Vec<String>) {
    STRS = Some(strs);
}

unsafe fn algorithm() {
    let strs = STRS.clone().unwrap();
    
    if strs.is_empty() {
        RESULT = Some(String::from(""));

        return;
    }

    let mut prefix: String = String::new();
    let strs_by_char: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let common_length = strs_by_char.iter().map(|s| s.len()).min().unwrap();

    for i in 0..common_length {
        let letter = strs_by_char[0][i];

        if !strs_by_char.iter().all(|s| s[i] == letter) {
            break;
        }
        
        prefix.push(letter);
    }

    RESULT = Some(prefix);

    return;
}

unsafe fn output() -> String {
    RESULT.clone().unwrap()
}


pub unsafe fn longest_common_prefix(strs: Vec<String>) -> String {
    input(strs);

    algorithm();

    output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let strs = vec_string!["flower", "flow", "flight"];

        assert_eq!(String::from("fl"), unsafe { longest_common_prefix(strs) });
    }

    #[test]
    fn example_two() {
        let strs = vec_string!["dog", "racecar", "car"];

        assert_eq!(String::from(""), unsafe { longest_common_prefix(strs) });
    }
}
