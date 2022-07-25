static mut NUMS: Option<&mut Vec<u32>> = None;
static mut RESULT: Option<u32> = None;

unsafe fn input(nums: &mut Vec<u32>) {
    NUMS = Some(&mut nums);
}

unsafe fn algorithm() {
    let nums = NUMS.clone().unwrap();
    let length = nums.len();

    if length == 0 {
        RESULT = Some(length as u32);
        
        return;
    }

    let mut temp_n = nums[0];
    let mut temp_length = 1;

    for i in 1..length {
        if nums[i] != temp_n {
            temp_n = nums[i];
            nums[temp_length] = nums[i];
            temp_length += 1;
        }
    }
    nums.resize(temp_length, 0);
    
    RESULT = Some(temp_length as u32);

    return;
    
}

unsafe fn output() -> u32 {
    RESULT.clone().unwrap()
}

pub unsafe fn remove_duplicates(nums: &mut Vec<u32>) -> u32 {
    input(nums);

    algorithm();

    output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 1, 2]; 

        assert_eq!(2, unsafe { remove_duplicates(&mut nums) });
        assert_eq!(vec![1, 2], nums);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]; 
        
        assert_eq!(5, unsafe { remove_duplicates(&mut nums) });
        assert_eq!(vec![0, 1, 2, 3, 4], nums)
    }
}
