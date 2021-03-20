//! **Mid Day Coding Challenge: Sorted Squared Array!**
//! 
//! Write a function that takes in a non-empty array of integers that are sorted 
//! in ascending order and returns a new array of the same length with the 
//! squares of the original integers also sorted in ascending order.
//! DO NOT: use any built-in sorting functions for the language.
//! 
//! Examples:
//! 
//! ```ignore
//! [1, 2, 3, 5, 6, 8, 9] -> [1, 4, 9, 25, 36, 64, 81]
//! [1] -> [1]
//! [0] -> [0]
//! [-2, -1] -> [1, 4]
//! [-5, -4, -3, -2, -1] -> [1, 4, 9, 16, 25]
//! [-10, -5, 0, 5, 10] -> [0, 25, 25, 100, 100]
//! [-50, -13, -2, -1, 0, 0, 1, 1, 2, 3, 19, 20] -> [0, 0, 1, 1, 1, 4, 4, 9, 169, 361, 400, 2500]
//! ```
//! 
//! Bonus:
//! Optimal Time and Optimal Space:
//! O(N) Time | O(N) space - where n is the length of the input array

use std::collections::VecDeque;

pub fn sort_and_square(input: &mut VecDeque<i32>) -> Vec<i32> {
    let mut output = VecDeque::new();
    let mut from_front: Option<i32> = None;
    let mut from_back: Option<i32> = None;

    loop {
        // Get a number from either end of the input
        if from_front.is_none() { from_front = input.pop_front(); }
        if from_back.is_none() { from_back = input.pop_back(); }

        // If we've exhausted tne input, break
        if from_front.is_none() || from_back.is_none() { break; }

        let front_abs = from_front.unwrap().abs();
        let back_abs = from_back.unwrap().abs();

        // Take the number with the greatest absolute value, square it, and prepend
        // to the output
        let val = if front_abs >= back_abs {
            from_front.take()
        } else {
            from_back.take()
        }.unwrap();

        output.push_front(val*val)
    }

    // When breaking from the loop, either the front or back (but not both) will
    // have a value, so push that to the front of the VecDeque.
    if let Some(val) = from_front { output.push_front(val*val); }
    if let Some(val) = from_back { output.push_front(val*val); }

    output.drain(0..output.len()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario1() {
        let mut input = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expected = vec![1, 4, 9, 16, 25, 36, 49, 64, 81];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario2() {
        let mut input = VecDeque::from(vec![1]);
        let expected = vec![1];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario3() {
        let mut input = VecDeque::from(vec![0]);
        let expected = vec![0];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario4() {
        let mut input = VecDeque::from(vec![-2, -1]);
        let expected = vec![1, 4];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario5() {
        let mut input = VecDeque::from(vec![-5, -4, -3, -2, -1]);
        let expected = vec![1, 4, 9, 16, 25];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario6() {
        let mut input = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expected = vec![1, 4, 9, 16, 25, 36, 49, 64, 81];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario7() {
        let mut input = VecDeque::from(vec![-10, -5, 0, 5, 10]);
        let expected = vec![0, 25, 25, 100, 100];
        assert_eq!(sort_and_square(&mut input), expected)
    }

    #[test]
    fn test_scenario8() {
        let mut input = VecDeque::from(vec![50, -13, -2, -1, 0, 0, 1, 1, 2, 3, 19, 20]);
        let expected = vec![0, 0, 1, 1, 1, 4, 4, 9, 169, 361, 400, 2500];
        assert_eq!(sort_and_square(&mut input), expected)
    }
}
