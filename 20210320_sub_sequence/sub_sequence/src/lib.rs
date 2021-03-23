//! # **VALIDATE SUBSEQUENCE:**
//! Given two non-empty arrays of integers, write a function that determines 
//! whether the second array is a subsequence of the first array.
//! 
//! A subsequence of an array is a set of numbers that aren't necessarily 
//! adjacent in the array but that are in the same order as they appear in the 
//! array. For instance, the numbers [1,3,4] form a subsequence of the array 
//! [1, 2, 3, 4] , and so do the numbers [2, 4] . Note that a single number in 
//! an array and the array itself are both valid subsequences of the array.
//! 
//! ## Examples:
//! 
//! ```
//! # use sub_sequence::is_valid_sub;
//! assert_eq!(is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[1, 6, -1, 10]), true);
//! assert_eq!(
//!     is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[5, 1, 22, 25, 6, -1, 8, 10]), 
//!     true
//! );
//! assert_eq!(is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[1, 6, 10]), true);
//! assert_eq!(is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[5, -1, 8, 10]), true);
//! assert_eq!(
//!     is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[5, 1, 22, 25, 6, -1, 8, 10, 12]), 
//!     false
//! );
//! assert_eq!(
//!     is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[4, 5, 1, 22, 25, 6, -1, 8, 10]), 
//!     false
//! );
//! assert_eq!(
//!     is_valid_sub(&[5, 1, 22, 25, 6, -1, 8, 10], &[5, 1, 22, 23, 6, -1, 8, 10]), 
//!     false
//! );
//! ```
//! 
//! Bonus: This should take O(N) time and O(1) space - where N is the length of 
//! the array. Good luck and have fun!


pub fn is_valid_sub<T: PartialEq>(seq: &[T], subseq: &[T]) -> bool {
    let mut sub_idx = 0;
    for item in seq {
        if *item == subseq[sub_idx] { 
            sub_idx += 1;
            if sub_idx == subseq.len() { return true; }    
        }
    }
    false
}
