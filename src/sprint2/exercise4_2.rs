#[derive(Clone)]
pub struct Solution {
    pub s: Option<String>,
    pub result: Option<i32>,
}

impl Default for Solution {
    fn default() -> Solution {
        Solution {
            s: Default::default(),
            result: Default::default()
        }
    }
}

fn input(s: String, solution: &mut Solution) {
    solution.s = Some(s);
}

fn algorithm(solution: &mut Solution) {
    let s = solution.s.clone().unwrap();
   
    if let Some(last) = s.split_whitespace().last() {
        solution.result = Some(last.len() as i32);
    } else {
        solution.result = Some(0);   
    }
}

fn output(solution: &Solution) -> i32 {
    solution.result.clone().unwrap()
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut solution: Solution = Solution::default();

    input(s, &mut solution);

    algorithm(&mut solution);

    output(&solution)
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
