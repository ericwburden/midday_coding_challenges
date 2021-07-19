// Given a sorted array, find the smallest positive integer that is not the sum of a
// subset of the array.
// 
// For example, for the input [1, 2, 3, 10], you should return 7.
// 
// Bonus: Do this in O(N) time.

pub enum SolutionType {
    Naive,
    Linear
}

pub fn min_impossible_sum(arr: &[u32], t: SolutionType) -> u32 {
    match t {
        SolutionType::Naive => min_impossible_sum_naive(arr),
        SolutionType::Linear => min_impossible_sum_linear(arr),
    }
}

/// Start by guessing `1`, then attempting to determine if the guess can be made by
/// a subset of `arr`. If it can, then increase the guess by one and trying again. 
/// Return `guess` when it can't be found from a subset of `arr`.
pub fn min_impossible_sum_naive(arr: &[u32]) -> u32 {
    fn can_sum(total: u32, arr: &[u32]) -> bool {
        if total == 0 { return true; }
        for n in arr {
            if *n > total { continue; }
            let new_total = total - n;
            let new_arr: Vec<_> = arr.iter().filter(|i| *i != n).cloned().collect();
            if can_sum(new_total, &new_arr) { return true; }
        }
        false
    }

    let mut guess = 1;
    while can_sum(guess, arr) { guess += 1; }
    guess
}

/// Start by guessing `1`. For each number in `arr`, if it's greater than our guess, 
/// we've found our result. Otherwise, increase guess by the number.j
pub fn min_impossible_sum_linear(arr: &[u32]) -> u32 {
    let mut guess = 1;
    for n in arr {
        if *n > guess { break; } else { guess += n; }
    }
    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(min_impossible_sum(&[5], SolutionType::Linear), 1);
        assert_eq!(min_impossible_sum(&[5], SolutionType::Naive), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(min_impossible_sum(&[1], SolutionType::Linear), 2);
        assert_eq!(min_impossible_sum(&[1], SolutionType::Naive), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(min_impossible_sum(&[1, 2, 3, 10], SolutionType::Linear), 7);
        assert_eq!(min_impossible_sum(&[1, 2, 3, 10], SolutionType::Naive), 7);
    }

    #[test]
    fn test4() {
        assert_eq!(min_impossible_sum(&[1, 2, 3, 7, 10], SolutionType::Linear), 24);
        assert_eq!(min_impossible_sum(&[1, 2, 3, 7, 10], SolutionType::Naive), 24);
    }
}
