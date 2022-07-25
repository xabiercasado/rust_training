pub fn remove_element(nums: &mut Vec<u32>, val: u32) -> u32 {
    let length = nums.len();
    let mut temp_length = 0;

    for i in 0..length {
        if nums[i] != val {
            nums[temp_length] = nums[i];
            temp_length += 1;
        }
        
    }
    nums.resize(temp_length, 0);
    
    temp_length as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![3, 2, 2, 3]; 
        let val = 3;

        assert_eq!(2, remove_element(&mut nums, val));
        assert_eq!(vec![2, 2], nums);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2]; 
        let val = 2;

        assert_eq!(5, remove_element(&mut nums, val));
        assert_eq!(vec![0, 1, 3, 0, 4], nums);
    }
}
