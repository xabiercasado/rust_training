#[derive(Clone)]
pub struct Solution {
    pub nums: Option<Vec<i32>>,
    pub result: Option<i32>,
}

impl Default for Solution {
    fn default() -> Solution {
        Solution {
            nums: Default::default(),
            result: Default::default()
        }
    }
}

fn input(nums: Vec<i32>, solution: &mut Solution) {
    solution.nums = Some(nums);
}

fn algorithm(solution: &mut Solution) {
    let nums = solution.nums.clone().unwrap();
    let mut prev = 0;
    let mut max = std::i32::MIN;
 
    for i in 0..nums.len() {
        prev = nums[i].max(prev + nums[i]);
        max = max.max(prev);
    }

    solution.result = Some(max);
}

fn output(solution: &Solution) -> i32 {
    solution.result.clone().unwrap()
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut solution: Solution = Solution::default();

    input(nums, &mut solution);

    algorithm(&mut solution);

    output(&solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        assert_eq!(6, max_sub_array(nums));
    }

    #[test]
    fn example_two() {
        let nums = vec![1];

        assert_eq!(1, max_sub_array(nums));
    }

    #[test]
    fn example_three() {
        let nums = vec![0];

        assert_eq!(0, max_sub_array(nums));
    }

    #[test]
    fn example_four() {
        let nums = vec![-1];

        assert_eq!(-1, max_sub_array(nums));
    }

    #[test]
    fn example_five() {
        let nums = vec![-100000];

        assert_eq!(-100000, max_sub_array(nums));
    }
}
