use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

/// Identify the possible permutations of a list of items
///
/// Given a list of values, identify all the different arrangements of those items. In cases
/// where an item is duplicated, resulting in multiple permutations with the same values in the
/// same order, de-duplicates the list of possible permutations.
///
/// Examples
///
/// ```no_run
/// permutations(&[1]) -> [[1]] // Only one possible arrangement
/// permutations(&['a', 'b']) -> [['a', 'b'], ['b', 'a']]
/// permutations(&[1, 2, 3]) -> [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
/// permutations(&[1, 1, 2]) -> [[2, 1, 1], [1, 2, 1], [1, 1, 2]]
/// ```
pub fn permutations<T: Clone + Eq + Hash>(list: &[T]) -> Vec<Vec<T>> {
    // The base case, if given a slice of length 1, return that slice as a vector
    if list.len() == 1 {
        return vec![list.to_vec()];
    }

    // Otherwise, prepare a vector large enough to hold the result, length list.len() factorial
    let ret_vec_len: usize = (1..=list.len()).product();
    let mut ret_vec: Vec<Vec<T>> = Vec::with_capacity(ret_vec_len);

    // For each item in `list`...
    for idx in 0..list.len() {
        let mut remaining_vec = list.to_vec(); // Convert to vector
        let n = remaining_vec.remove(idx); // "Pop" out the item at `idx`

        // Recursively call `permutations()` with the remaining vector items
        let mut sub_vecs = permutations(&remaining_vec);

        // Append `n` to each sub-vector
        for v in sub_vecs.iter_mut() {
            v.push(n.clone());
        }
        ret_vec.extend(sub_vecs); // Add all the sub-vectors to the return vector
    }

    // De-duplicate the permutations before returning
    let ret_vec: Vec<Vec<T>> = HashSet::<_>::from_iter(ret_vec.into_iter())
        .into_iter()
        .collect();

    ret_vec
}

#[cfg(test)]
mod tests {
    use super::{FromIterator, Hash, HashSet};
    use crate::permutations;

    // Test helper function to determine if the expected number of permutations was found
    fn len_matches<T: Eq + Hash>(l: &[T], v: &[Vec<T>]) -> bool {
        let desired_len: usize = (1..=l.len()).product();
        let unique_vals: HashSet<_> = HashSet::from_iter(v.iter());
        desired_len == unique_vals.len()
    }

    // Test helper function to determine if the permutations contain all the items from the
    // original slice
    fn elements_match<T: Clone + Ord>(l: &[T], v: &[Vec<T>]) -> bool {
        let l_sorted = l.to_vec().sort_unstable();
        for sub_vec in v {
            let sub_vec_sorted = sub_vec.to_vec().sort_unstable();
            if l_sorted != sub_vec_sorted {
                return false;
            }
        }
        true
    }

    fn confirm_permutations<T: Clone + Ord + Eq + Hash>(l: &[T], v: Vec<Vec<T>>) -> bool {
        len_matches(l, &v) && elements_match(l, &v)
    }

    /// Test and confirm that `permutations()` returns the expected values when given a
    /// slice containing unique values.
    #[test]
    fn check_unique_values() {
        assert!(confirm_permutations(&[1], permutations(&[1])));
        assert!(confirm_permutations(&[1, 2], permutations(&[1, 2])));
        assert!(confirm_permutations(&[1, 2, 3], permutations(&[1, 2, 3])));

        assert!(confirm_permutations(
            &['a', 'b', 'c'],
            permutations(&['a', 'b', 'c'])
        ));

        assert!(confirm_permutations(
            &["this", "is", "great"],
            permutations(&["this", "is", "great"])
        ));
    }

    /// Test and confirm that `permutations()` returns the expected values when given a
    /// slice containing values where some values may be duplicated.
    #[test]
    fn check_with_duplicates() {
        let list = &[1, 1, 2];

        let mut expected_result = Vec::new();
        expected_result.push(vec![2, 1, 1]);
        expected_result.push(vec![1, 2, 1]);
        expected_result.push(vec![1, 1, 2]);
        expected_result.sort_unstable();

        let mut actual_result = permutations(list);
        actual_result.sort_unstable();

        assert_eq!(actual_result, expected_result);
    }
}
