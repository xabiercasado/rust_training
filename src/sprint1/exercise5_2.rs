static mut NUMS: Option<Vec<u32>> = None;
static mut VAL: Option<u32> = None;
static mut RESULT: Option<u32> = None;

struct Solution {
    nums: &mut Vec<u32>,
    val: u32,
    result: u32,
}

unsafe fn input(nums: &mut Vec<u32>, val: u32) {
    Solution.nums = nums;
    VAL = Some(val);
}

unsafe fn algorithm() {
    let mut nums = NUMS.unwrap().iter().cloned().collect::<Vec<_>>();

    let val = VAL.clone().unwrap();
    let length = nums.len();
    let mut temp_length = 0;

    for i in 0..length {
        if nums[i] != val {
            nums[temp_length] = nums[i];
            temp_length += 1;
        }

    }
    nums.resize(temp_length, 0);
    
    NUMS = Some(nums);

    RESULT = Some(temp_length as u32);
    
    return;
}

unsafe fn output() -> u32 {
    RESULT.clone().unwrap()
}

pub unsafe fn remove_element(nums: &mut Vec<u32>, val: u32) -> u32 {
    input(&mut nums, val);

    algorithm();

    output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![3, 2, 2, 3]; 
        let val = 3;

        assert_eq!(2, unsafe { remove_element(&mut nums, val) });
        assert_eq!(vec![2, 2], nums);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2]; 
        let val = 2;

        assert_eq!(5, unsafe { remove_element(&mut nums, val) });
        assert_eq!(vec![0, 1, 3, 0, 4], nums);
    }
}
