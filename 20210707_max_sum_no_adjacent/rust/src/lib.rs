//! # Max Subset Sum, No Adjacent:
//!
//! Write a function that takes in an array of positive integers and returns the
//! maximum sum of non-adjacent elements in the array. If the input array is empty, the
//! function should return 0.
//!
//! # Business Rules / Errata:
//!
//! - Don't sort the input array
//! - You should return a Maximum sum that can be derived from any combination of
//!   non-adjacent numbers, even if that comes from a single value in the array
//!   (consider the sum of an array of length 1 == the numeric in the array).
//!
//! # Test Cases
//!
//! ```ignore
//! max_subset_sum_no_adjacent([75, 105, 120, 75, 90, 135]) -> 330 # 75 + 120 + 135
//! max_subset_sum_no_adjacent([]) -> 0
//! max_subset_sum_no_adjacent([1]) -> 1
//! max_subset_sum_no_adjacent([1, 2, 3]) -> 4 # 1 + 3
//! max_subset_sum_no_adjacent([7, 10, 12, 7, 9, 14]) -> 33 # 7 + 12 + 14
//!
//! # 2 should not be summed with 1 because they are adjacent, and 2 is the highest value
//! max_subset_sum_no_adjacent([1, 2]) -> 2
//!
//! # This is a tricky one, since 15 is greater than 1 + 3, 15 is the max subset sum
//! max_subset_sum_no_adjacent([1, 15, 3]) -> 15
//!
//! # 4 + 200 + 3 // (array[0] + array[3] + array[5]), again this is tricky
//! max_subset_sum_no_adjacent([4, 3, 5, 200, 5, 3]) -> 207
//!
//! # 10 + 20 + 15 + 15 // (array[0] + array[2] + array[4] + array[7])
//! max_subset_sum_no_adjacent([10, 5, 20, 25, 15, 5, 5, 15]) -> 60
//!
//! # 10 + 20 + 15 + 15 + 15 + 15 // (array[0] + array[2] + array[4] + array[7] + array[9] + array[12])
//! max_subset_sum_no_adjacent([10, 5, 20, 25, 15, 5, 5, 15, 3, 15, 5, 5, 15]) -> 90
//! ```

/// Maximum sum of a non-adjacent subset
///
/// Given a slice of integers <u32>, identify the maximum value that could be derived
/// from a sum of non-adjacent values.
pub fn max_subset_sum_no_adjacent(nums: &[u32]) -> u32 {
    // Inner recursive function. Given a current sum and a slice of integers, adds the
    // first value to the slice, appends that value to `sums`, then recurses over
    // the remaining values, skipping the first (non-adjacency). No need for memoization
    // here, since the recursion only moves forward in the input array.
    fn inner_rec(sum: u32, nums: &[u32], sums: &mut Vec<u32>) {
        // Just return if `nums` is empty
        let (val, nums) = match nums.split_first() { Some(x) => x, None => return, };
        let new_sum = sum + val;
        sums.push(new_sum); // Add to the running list of sums

        // Start at index 1 to skip the adjacent item
        (1..nums.len()).for_each(|idx| inner_rec(new_sum, &nums[idx..], sums))
    }

    let mut sums = Vec::new(); // A running record of all sums

    // Starting at each index in `nums`, calculate all non-adjacent sums
    nums.iter().enumerate().for_each(|(idx, _)| inner_rec(0, &nums[idx..], &mut sums));
    sums.iter().fold(0, |max, v| if *v > max { *v } else { max })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_output(input: &[u32], expected: u32) {
        assert_eq!(max_subset_sum_no_adjacent(input), expected);
    }
    
    #[test]
    fn test1() { check_output(&[75, 105, 120, 75, 90, 135], 330); }

    #[test]
    fn test2() { check_output(&[], 0); }

    #[test]
    fn test3() { check_output(&[1], 1); }

    #[test]
    fn test4() { check_output(&[1, 2], 2); }

    #[test]
    fn test5() { check_output(&[1, 2, 3], 4); }

    #[test]
    fn test6() { check_output(&[1, 15, 3], 15); }

    #[test]
    fn test7() { check_output(&[7, 10, 12, 7, 9, 14], 33); }

    #[test]
    fn test8() { check_output(&[4, 3, 5, 200, 5, 3], 207); }

    #[test]
    fn test9() { check_output(&[10, 5, 20, 25, 15, 5, 5, 15], 60); }

    #[test]
    fn test10() { check_output(&[10, 5, 20, 25, 15, 5, 5, 15, 3, 15, 5, 5, 15], 90); }
}
