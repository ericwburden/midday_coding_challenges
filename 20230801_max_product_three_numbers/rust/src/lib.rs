#![allow(dead_code)]

/// Given a list of values, return the largest possible product of `k` of
/// those values. If there aren't enough values, returns `None`.
fn largest_possible_product(input: &[i32], k: usize) -> Option<i32> {
    // Check to be sure there are at least `k` values in `input`.
    if k > input.len() {
        return None;
    }

    // Make a copy of the input slice and sort it.
    let mut sorted_input = input.to_vec();
    sorted_input.sort_unstable();

    // Create an empty vector to hold the values that will serve as factors
    // for the final product. Set two indices to point at the first and last
    // indices of the sorted input.
    let mut output = Vec::with_capacity(k);
    let mut p1 = 0;
    let mut p2 = sorted_input.len() - 1;

    // Until we've selected enough factors...
    while output.len() < k {
        // Select the smallest two values and the largest two values from the
        // sorted input that haven't already been selected as factors.
        let (small_n1, small_n2) = (sorted_input.get(p1)?, sorted_input.get(p1 + 1)?);
        let (large_n1, large_n2) = (sorted_input.get(p2)?, sorted_input.get(p2 - 1)?);

        if small_n1 * small_n2 > large_n1 * large_n2 && output.len() < k - 1 {
            // If the product of the two smallest values is larger than the
            // product of the two largest values (because they're both negative)
            // AND there's at least two spaces left for factors, add both of the
            // two "small" values as factors and move the beginning pointer
            // forward so that these values aren't selected again.
            output.push(small_n1);
            output.push(small_n2);
            p1 += 2;
        } else {
            // Otherwise, add the largest value as a factor and move the ending
            // pointer backward so that this value can't be selected again.
            output.push(large_n1);
            p2 -= 1;
        }
    }

    // Return the product of all the factors.
    Some(output.into_iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_product() {
        let input1 = vec![-4, -4, -4, -4, -4, -4, -4];
        let input2 = vec![-4, -4, -4, -4, -4, -4, 2];
        let input3 = vec![-4, -4, -4, -4, -4, 2, 2];
        let input4 = vec![-4, -4, -4, -4, 2, 2, 2];
        let input5 = vec![-4, -4, -4, 1, 2, 2, 2];
        let input6 = vec![-4, -4, 1, 1, 2, 2, 2];
        let input7 = vec![-4, 1, 1, 1, 2, 2, 2];
        let input8 = vec![1, 1, 1, 1, 2, 2, 2];

        assert_eq!(largest_possible_product(&input1, 3), Some(-64));
        assert_eq!(largest_possible_product(&input2, 3), Some(32));
        assert_eq!(largest_possible_product(&input3, 3), Some(32));
        assert_eq!(largest_possible_product(&input4, 3), Some(32));
        assert_eq!(largest_possible_product(&input5, 3), Some(32));
        assert_eq!(largest_possible_product(&input6, 3), Some(32));
        assert_eq!(largest_possible_product(&input7, 3), Some(8));
        assert_eq!(largest_possible_product(&input8, 3), Some(8));

        assert_eq!(largest_possible_product(&input1, 4), Some(256));
        assert_eq!(largest_possible_product(&input2, 4), Some(256));
        assert_eq!(largest_possible_product(&input3, 4), Some(256));
        assert_eq!(largest_possible_product(&input4, 4), Some(256));
        assert_eq!(largest_possible_product(&input5, 4), Some(64));
        assert_eq!(largest_possible_product(&input6, 4), Some(64));
        assert_eq!(largest_possible_product(&input7, 4), Some(8));
        assert_eq!(largest_possible_product(&input8, 4), Some(8));

        assert_eq!(largest_possible_product(&input1, 5), Some(-1024));
        assert_eq!(largest_possible_product(&input2, 5), Some(512));
        assert_eq!(largest_possible_product(&input3, 5), Some(512));
        assert_eq!(largest_possible_product(&input4, 5), Some(512));
        assert_eq!(largest_possible_product(&input5, 5), Some(128));
        assert_eq!(largest_possible_product(&input6, 5), Some(128));
        assert_eq!(largest_possible_product(&input7, 5), Some(8));
        assert_eq!(largest_possible_product(&input8, 5), Some(8));
    }
}
