//! Mid-Day Coding Challenge
//! 
//! You are given an array of non-negative integers. Let's say you start at the
//! beginning of the array and are trying to advance to the end. You can advance
//! at most, the number of steps corresponding to the number that you're
//! currently on. (For example, if the number at your current index is 3, you
//! can advance 1, 2, or 3 steps.) Determine whether you can get to the end of
//! the array.
//! 
//! For example, given the array [1, 3, 1, 2, 0, 1], we can go from indices
//! 0 -> 1 -> 3 -> 5, so return true.
//! 
//! Given the array [1, 2, 1, 0, 0], we can't reach the end, so return false.
//! 
//! Write a function that can take an array of arbitrary length and determine
//! whether the array can be walked to the end as described.

pub fn can_walk_array(arr: &[usize]) -> bool {
    if arr.len() == 1 { return true; }
    if arr[0] == 0 { return false; }
    (1..=arr[0]).any(|x| can_walk_array(&arr[x..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_example() {
        let input = [1, 3, 1, 2, 0, 1];
        assert_eq!(can_walk_array(&input), true)
    }

    #[test]
    fn test_failing_example() {
        let input = [1, 2, 1, 0, 0];
        assert_eq!(can_walk_array(&input), false)
    }
}
