mod easy;
mod sprint1;

use leetcode_prelude::vec_string;

fn main() {
    easy::exercise1_1::two_sum(vec![2, 7, 11, 15], 9);
    unsafe { easy::exercise1_2::two_sum(vec![2, 7, 11, 15], 9); };
    easy::exercise2_1::is_palindrome(121);

    sprint1::exercise1_1::roman_to_int(String::from("III"));
    unsafe { sprint1::exercise1_2::roman_to_int(String::from("III")) };
    sprint1::exercise2_1::longest_common_prefix(vec_string!["flower", "flow", "flight"]);
    unsafe { sprint1::exercise2_2::longest_common_prefix(vec_string!["flower", "flow", "flight"]) };
    sprint1::exercise3_1::is_valid(String::from("()"));
    unsafe { sprint1::exercise3_2::is_valid(String::from("()")) };
    sprint1::exercise4_1::remove_duplicates(&mut vec![1, 1, 2]);
    sprint1::exercise5_1::remove_element(&mut vec![3, 2, 2, 3], 3);
}
