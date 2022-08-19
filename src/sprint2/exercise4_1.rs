pub fn length_of_last_word(s: String) -> i32 {
    if let Some(last) = s.split_whitespace().last() {
        return last.len() as i32;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let s = String::from("Hello World");

        assert_eq!(5, length_of_last_word(s));
    }

    #[test]
    fn example_two() {
        let s = String::from(" ");

        assert_eq!(0, length_of_last_word(s));
    }
}
