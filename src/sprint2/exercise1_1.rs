pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(i) => i as i32,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let haystack = String::from("hello");
        let needle = String::from("ll");

        assert_eq!(2, str_str(haystack, needle));
    }

    #[test]
    fn example_two() {
        let haystack = String::from("aaaaa");
        let needle = String::from("bba");

        assert_eq!(-1, str_str(haystack, needle));
    }

    #[test]
    fn example_three() {
        let haystack = String::from("");
        let needle = String::from("");

        assert_eq!(0, str_str(haystack, needle));
    }
}
