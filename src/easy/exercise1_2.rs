use std::collections::HashMap;

static mut NUMS: Option<Vec<i32>> = None;
static mut TARGET: Option<i32> = None;
static mut RESULT: Option<Vec<i32>> = None;

unsafe fn input(nums: Vec<i32>, target: i32) {
    NUMS = Some(nums);
    TARGET = Some(target);
}

unsafe fn algorithm() {
    let nums = NUMS.clone().unwrap();
    let target = TARGET.unwrap();
    
    let mut result: HashMap<i32, i32> = HashMap::new();

    for a in 0..nums.len() {
        if let Some(&b) = result.get(&(target - nums[a])) {
            RESULT = Some(vec![b, a as i32]);
            
            return;
        }

        result.insert(nums[a], a as i32);
    }

    RESULT = Some(vec![]);
    
    return;
}

unsafe fn output() -> Vec<i32> {
    // RESULT = Some(result);
    RESULT.clone().unwrap()
}

pub unsafe fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    input(nums, target);

    algorithm();
    
    // let result = output();
    // result
    output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(vec![0,1], unsafe { two_sum(nums, target) });
    }

    #[test]
    fn example_two() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(vec![1,2], unsafe { two_sum(nums, target) });
    }

    #[test]
    fn example_three() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(vec![0,1], unsafe { two_sum(nums, target) });
    }
}
