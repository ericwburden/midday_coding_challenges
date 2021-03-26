//! # CODE CHALLENGE
//! 
//! Given an array of integers a, your task is to calculate the digits that 
//! occur the most number of times in the array. Return the array of these digits 
//! in ascending order.
//! 
//! ## Example
//! 
//! For a = [25, 2, 3, 57, 38, 41], the output should be 
//! mostFrequentDigits(a) = [2, 3, 5].
//! Here are the number of times each digit appears in the array:
//! 0 -> 0
//! 1 -> 1
//! 2 -> 2
//! 3 -> 2
//! 4 -> 1
//! 5 -> 2
//! 6 -> 0
//! 7 -> 1
//! 8 -> 1
//! 
//! The most number of times any number occurs in the array is 2, and the digits 
//! which appear 2 times are 2, 3 and 5. So the answer is [2, 3, 5]. 

use std::collections::HashMap;
use std::hash::Hash;

/// Given a slice of `usize` items, returns the digits that appear the most often
/// in any integer. For example, the most frequent digit in [123, 255, 45] is '5'.
pub fn most_frequent_digits(input: &[usize]) -> Vec<usize> {
    // HashMap<digit, number of appearances>
    let digit_frequency_map = map_digit_frequency(input); 

    let max_frequency = digit_frequency_map.values().max().map_or(0, |n| *n);
    let mut most_frequent_digits = digit_frequency_map
        .iter()
        .filter(|(_, v)| *v == &max_frequency)
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();
    most_frequent_digits.sort_unstable();
    most_frequent_digits
}

/// Given a slice of `usize` items, builds a mapping of digits to the number of
/// times that digit appears in the list
pub fn map_digit_frequency(input: &[usize]) -> HashMap<usize, usize> {
    let mut output = HashMap::new();
    input
        .iter()
        .map(|n| to_digits(*n))
        .flatten()
        .for_each(|n| output.upsert(n));

    output
}

/// Given a `usize`, returns an iterator over the digits in that integer
pub fn to_digits(mut n: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while n >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = n / divisor;
            n %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

/// Trait implementing an `upsert` method, used to either add a number to a digit
/// frequency mapping or increase the current value if that digit already exists
/// in the mapping.
pub trait Upsert<T: Hash + Eq> {
    fn upsert(&mut self, item: T);
}

impl<T: Hash + Eq> Upsert<T> for HashMap<T, usize> {
    fn upsert(&mut self, item: T) {
        *self.entry(item).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_frequency_mapping() {
        let input = [25, 2, 3, 57, 38, 41];
        let output = map_digit_frequency(&input);

        assert_eq!(output.get(&0).map_or(0, |n| *n), 0);
        assert_eq!(output.get(&1).map_or(0, |n| *n), 1);
        assert_eq!(output.get(&2).map_or(0, |n| *n), 2);
        assert_eq!(output.get(&3).map_or(0, |n| *n), 2);
        assert_eq!(output.get(&4).map_or(0, |n| *n), 1);
        assert_eq!(output.get(&5).map_or(0, |n| *n), 2);
        assert_eq!(output.get(&6).map_or(0, |n| *n), 0);
        assert_eq!(output.get(&7).map_or(0, |n| *n), 1);
        assert_eq!(output.get(&8).map_or(0, |n| *n), 1);
        assert_eq!(output.get(&9).map_or(0, |n| *n), 0);
    }

    #[test]
    fn test_most_frequent_digits() {
        let input = [25, 2, 3, 57, 38, 41];
        let output = most_frequent_digits(&input);

        assert_eq!(output, vec![2, 3, 5]);
    }
}
