pub fn remove_duplicates(nums: &mut Vec<u32>) -> u32 {
    let length = nums.len();

    if length == 0 {
        return length as u32;
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
    
    temp_length as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 1, 2]; 

        assert_eq!(2, remove_duplicates(&mut nums));
        assert_eq!(vec![1, 2], nums);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]; 
        
        assert_eq!(5, remove_duplicates(&mut nums));
        assert_eq!(vec![0, 1, 2, 3, 4], nums)
    }
}
