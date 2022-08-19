mod easy;
mod sprint1;
mod sprint2;

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
    // unsafe { sprint1::exercise4_2::remove_duplicates(&mut vec![1, 1, 2]) };
    sprint1::exercise5_1::remove_element(&mut vec![3, 2, 2, 3], 3);
    // unsafe { sprint1::exercise5_2::remove_element(&mut vec![3, 2, 2, 3], 3) };

    sprint2::exercise1_1::str_str(String::from("hello"), String::from("ll"));
    sprint2::exercise1_2::str_str(String::from("hello"), String::from("ll"));
    sprint2::exercise2_1::search_insert(vec![1, 3, 5, 6], 5);
    sprint2::exercise2_2::search_insert(vec![1, 3, 5, 6], 5);
    sprint2::exercise3_1::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    sprint2::exercise3_2::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    sprint2::exercise4_1::length_of_last_word(String::from("Hello World"));
    sprint2::exercise4_2::length_of_last_word(String::from("Hello World"));
}
