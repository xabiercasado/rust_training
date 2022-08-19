pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut max = std::i32::MIN;
    
    for i in 0..nums.len() {
        prev = nums[i].max(prev + nums[i]);
        max = max.max(prev);
    }

    max
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
