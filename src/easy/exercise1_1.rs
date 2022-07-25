// use std::env;
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    
    /* 
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = result.get(&(target - num)) {
            return result.get(&(target - nums[a]));
        } else {
            result.insert(num, i as i32);
        }
    }
    */
    
    for a in 0..nums.len() {
        // println!("nums[{}] = {}", a, nums[a]);
        
        /* 
         * TODO: Correct the 'None' case
        match result.get(&(target - nums[a])) {
            Some(&b) => vec![a as i32, b],
            _ => {},
        }
        */

        if let Some(&b) = result.get(&(target - nums[a])) {
            return vec![b, a as i32];
        }

        result.insert(nums[a], a as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(vec![0,1], two_sum(nums, target));
    }

    #[test]
    fn example_two() {
        let nums = vec![3, 2, 4];
        let target = 6;
        
        assert_eq!(vec![1,2], two_sum(nums, target));
    }

    #[test]
    fn example_three() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(vec![0,1], two_sum(nums, target));
    }
}
