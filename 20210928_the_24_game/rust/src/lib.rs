/// Handles the division edge cases
fn div(a: i32, b:i32) -> i32 {
    if a == 0 || b == 0 { return 0; }
    (a as f64 / b as f64).round() as i32
}

/// Performs all four operations on a given pair and returns an array of results
fn all_operations(a: i32, b: i32) -> [i32; 4] {
    [a + b, a - b, a * b, div(a, b)]
}

fn play(nums: &[i32]) -> bool {
    if nums.len() == 1 { return nums[0] == 24; } // Base case

    // For each pair of values in `nums`...
    for (idx, vals) in nums.windows(2).enumerate() {
        // For each result from all four operations on each pair of `nums`...
        for result in all_operations(vals[0], vals[1]).iter() {
            // Prepare a new set of numbers containing the numbers before and after
            // the pair operated on, plus the result of the operation
            let mut new_nums = nums.to_vec();
            new_nums.splice(idx..=idx+1, std::iter::once(*result));
            
            // Check the new set
            if play(&new_nums) { return true; }
        }
    }
    return false;   // If no passing cases are found, return false
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn should_pass_all_the_repo_test_cases() {
        let test_cases = [
            ([5, 2, 7, 8],  true),
            ([2, 4, 8, 10], true),
            ([27, 1, 1, 1], true),
            ([5, 0, 4, 4],  true),
            ([47, 2, 0, 0], true),
            ([1, 1, 73, 3], true),
            ([1, 5, 7, 19], false),
        ];
        for (input, result) in test_cases.iter() {
            assert_eq!(play(input), *result);
        }
    }

    #[test]
    fn test_handles_simplest_case() {
        // 2 + 4 + 8 + 10 = 24
        let nums = [2, 4, 8, 10];
        assert!(play(&nums));
    }

    #[test]
    fn test_handles_div_by_zero() {
        // (5 + 0) * 4 + 4 = 24
        let nums = [5, 0, 4, 4];
        assert!(play(&nums))
    }

    #[test]
    fn test_handles_integer_division_rounding() {
        // 47 / 2 = 23.5
        // round(23.5) -> 24
        // trunc(23.5) -> 23
        let nums = [47, 2, 0, 0];
        assert!(play(&nums));
    }

    #[test]
    fn test_identifies_failing_combinations() {
        // Cannot make 24 with these numbers
        let nums = [1, 5, 7, 19];
        assert!(!play(&nums));
    }
}
