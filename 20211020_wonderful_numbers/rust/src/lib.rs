// You are given a string num, representing a large integer, and an integer k.
// We call some integer wonderful if it is a permutation of the digits in num and is
// greater in value than num. There can be many wonderful integers. However, we only
// care about the smallest-valued ones.
//
// For example, when num = "5489355142":
// The 1st smallest wonderful integer is "5489355214".
// The 2nd smallest wonderful integer is "5489355241".
// The 3rd smallest wonderful integer is "5489355412".
// The 4th smallest wonderful integer is "5489355421".
//
// Return the minimum number of adjacent digit swaps that needs to be applied to num
// to reach the kth smallest wonderful integer.
//
// The tests are generated in such a way that kth smallest wonderful integer exists.
// Example 1:
// Input: num = "5489355142", k = 4
// Output: 2
// Explanation: The 4th smallest wonderful number is "5489355421". To get this number:
// - Swap index 7 with index 8: "5489355142" -> "5489355412"
// - Swap index 8 with index 9: "5489355412" -> "5489355421"
//
// Example 2:
// Input: num = "11112", k = 4
// Output: 4
// Explanation: The 4th smallest wonderful number is "21111". To get this number:
// - Swap index 3 with index 4: "11112" -> "11121"
// - Swap index 2 with index 3: "11121" -> "11211"
// - Swap index 1 with index 2: "11211" -> "12111"
// - Swap index 0 with index 1: "12111" -> "21111"
//
// Example 3:
// Input: num = "00123", k = 1
// Output: 1
// Explanation: The 1st smallest wonderful number is "00132". To get this number:
// - Swap index 3 with index 4: "00123" -> "00132"
//
// Constraints:
// - 2 <= num.length <= 1000
// - 1 <= k <= 1000
// - num only consists of digits.

use std::convert::From;

/// Iterator that yields permutations of the input vector, in lexicographic
/// order, starting with the order of the original input.
struct PermutingIterator<T>(Vec<T>);

impl<T: Ord + Copy> Iterator for PermutingIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() < 2 {
            return None;
        }

        for idx in (0..self.0.len() - 1).rev() {
            if self.0[idx] < self.0[idx + 1] {
                let sub_idx = self.0[idx..]
                    .iter()
                    .rposition(|x| self.0[idx] < *x)
                    .unwrap();
                self.0.swap(idx, sub_idx + idx);
                self.0[idx + 1..].reverse();
                return Some(self.0.clone());
            }
        }
        None
    }
}

impl From<&str> for PermutingIterator<char> {
    fn from(value: &str) -> Self {
        PermutingIterator(value.chars().collect())
    }
}

fn swaps_to_kth_permutation(num: &str, k: usize) -> usize {
    let mut permuting_iterator = PermutingIterator::from(num);
    let kth_permutation = permuting_iterator.nth(k-1).unwrap();

    // Iterator chain that collects the indices that differ between `num` and 
    // its kth permutation in sequence.
    let differing_indices: Vec<_> = num
        .chars()
        .zip(kth_permutation.iter())
        .enumerate()
        .filter(|(_, (lchar, rchar))| lchar != *rchar)
        .map(|(idx, _)| idx)
        .collect();
    differing_indices.last().unwrap() - differing_indices[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_find_next_lexicographic_permutation() {
        let input = "5489355142".chars().collect();
        let mut permuting_iterator = PermutingIterator(input);
        assert_eq!(
            permuting_iterator.next(),
            Some(vec!['5', '4', '8', '9', '3', '5', '5', '2', '1', '4'])
        );
        assert_eq!(
            permuting_iterator.next(),
            Some(vec!['5', '4', '8', '9', '3', '5', '5', '2', '4', '1'])
        );
        assert_eq!(
            permuting_iterator.next(),
            Some(vec!['5', '4', '8', '9', '3', '5', '5', '4', '1', '2'])
        );
        assert_eq!(
            permuting_iterator.next(),
            Some(vec!['5', '4', '8', '9', '3', '5', '5', '4', '2', '1'])
        );

        let input = "123".chars().collect();
        let mut permuting_iterator = PermutingIterator(input);
        assert_eq!(permuting_iterator.next(), Some(vec!['1', '3', '2']));
        assert_eq!(permuting_iterator.next(), Some(vec!['2', '1', '3']));
        assert_eq!(permuting_iterator.next(), Some(vec!['2', '3', '1']));
        assert_eq!(permuting_iterator.next(), Some(vec!['3', '1', '2']));
        assert_eq!(permuting_iterator.next(), Some(vec!['3', '2', '1']));
        assert_eq!(permuting_iterator.next(), None);
    }

    #[test]
    fn should_be_able_to_count_adjacent_swaps_to_kth_permutation() {
        let input = "5489355142";
        let result = swaps_to_kth_permutation(input, 4);
        assert_eq!(result, 2);

        let input = "11112";
        let result = swaps_to_kth_permutation(input, 4);
        assert_eq!(result, 4);

        let input = "00123";
        let result = swaps_to_kth_permutation(input, 1);
        assert_eq!(result, 1);
    }
}
