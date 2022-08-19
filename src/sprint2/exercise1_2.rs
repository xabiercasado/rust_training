#[derive(Clone)]
pub struct Solution {
    pub haystack: Option<String>,
    pub needle: Option<String>,
    pub result: Option<i32>,
}

impl Default for Solution {
    fn default() -> Solution {
        Solution {
            haystack: Default::default(),
            needle: Default::default(),
            result: Default::default()
        }
    }
}

fn input(haystack: String, needle: String, solution: &mut Solution) {
    solution.haystack = Some(haystack);
    solution.needle = Some(needle);
}

fn algorithm(solution: &mut Solution) {
    let haystack = solution.haystack.clone().unwrap();
    let needle = solution.needle.clone().unwrap();

    match haystack.find(&needle) {
        Some(i) => solution.result = Some(i as i32),
        _ => solution.result = Some(-1),
    }
}

fn output(solution: &Solution) -> i32 {
    solution.result.clone().unwrap()
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut solution: Solution = Solution::default();

    input(haystack, needle, &mut solution);

    algorithm(&mut solution);

    output(&solution)
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
