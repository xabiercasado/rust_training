pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(i) => i as i32,
        Err(i) => i as i32,
    }
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
