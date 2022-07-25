use leetcode_prelude::vec_string;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
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

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let strs = vec_string!["flower", "flow", "flight"];

        assert_eq!(String::from("fl"), longest_common_prefix(strs));
    }

    #[test]
    fn example_two() {
        let strs = vec_string!["dog", "racecar", "car"];

        assert_eq!(String::from(""), longest_common_prefix(strs));
    }
}