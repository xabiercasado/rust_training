#[derive(Clone)]
pub struct Solution {
    pub nums: Option<Vec<i32>>,
    pub target: Option<i32>,
    pub result: Option<i32>,
}

impl Default for Solution {
    fn default() -> Solution {
        Solution {
            nums: Default::default(),
            target: Default::default(),
            result: Default::default()
        }
    }
}

fn input(nums: Vec<i32>, target: i32, solution: &mut Solution) {
    solution.nums = Some(nums);
    solution.target = Some(target);
}

fn algorithm(solution: &mut Solution) {
    let nums = solution.nums.clone().unwrap();
    let target = solution.target.clone().unwrap();

    match nums.binary_search(&target) {
        Ok(i) => solution.result = Some(i as i32),
        Err(i) => solution.result = Some(i as i32),
    }
}

fn output(solution: &Solution) -> i32 {
    solution.result.clone().unwrap()
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut solution: Solution = Solution::default();

    input(nums, target, &mut solution);

    algorithm(&mut solution);

    output(&solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(2, search_insert(nums, target));
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;

        assert_eq!(1, search_insert(nums, target));
    }

    #[test]
    fn example_three() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;

        assert_eq!(4, search_insert(nums, target));
    }

    #[test]
    fn example_four() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;

        assert_eq!(0, search_insert(nums, target));
    }

    #[test]
    fn example_five() {
        let nums = vec![1];
        let target = 0;

        assert_eq!(0, search_insert(nums, target));
    }
}
