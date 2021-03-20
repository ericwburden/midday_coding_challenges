//! **MIDDAY CODING CHALLENGE**: Determine if the sum of two integers is equal 
//! to the given value
//! 
//! Given an array of integers and a value, determine if there are any two 
//! integers in the array whose sum is equal to the given value. Return true if 
//! the sum exists and return false if it does not.
//! 
//! Example:
//! 
//! ```
//! # use find_sum_of_two::find_sum_of_two;
//! let arr = [5, 7, 1, 2, 8, 4, 3];
//! assert_eq!(find_sum_of_two(&arr, 10), true);  // True -> 7+3=10, 8+2=10
//! assert_eq!(find_sum_of_two(&arr, 19), false); // False -> No two numbers = 19
//! assert_eq!(find_sum_of_two(&arr, 12), true);  // True -> 8+4=12 
//! ```

use std::collections::HashSet;

pub fn find_sum_of_two(arr: &[i32], total: i32) -> bool {
    let mut numbers_tried = HashSet::new();
    for number in arr {
        let diff = total - number;
        if numbers_tried.contains(&diff) { return true; }
        numbers_tried.insert(number);
    }

    false
}
