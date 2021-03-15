pub fn sorted_merge<T: Ord + Copy>(lhs: &[T], rhs: &[T]) -> Vec<T> {
    let (mut lhs_idx, mut rhs_idx) = (0, 0);
    let mut output = Vec::with_capacity(lhs.len() + rhs.len());
    
    // Until we've exhausted one of the input slices, check the order of the two values
    // at the two current indices. Whichever one is lower (ties go to the left-hand-side),
    // add that value to the output Vec and increment that side's index counter.
    loop {
        let (lhs_val, rhs_val) = (lhs[lhs_idx], rhs[rhs_idx]);
        match lhs_val <= rhs_val {
            true => {
                output.push(lhs_val);
                lhs_idx += 1;
            },
            false => {
                output.push(rhs_val);
                rhs_idx += 1;
            }
        }

        // If either side is exhausted, add all the remaining elements from the other side
        // and break the loop.
        if lhs_idx >= lhs.len() {
            output.extend(&rhs[rhs_idx..]);
            break;
        }
        if rhs_idx >= rhs.len() {
            output.extend(&lhs[lhs_idx..]);
            break;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_my_work() {
        let arr1 = vec![1, 9, 12, 16];
        let arr2 = vec![4, 10, 15, 20];
        let expected = vec![1, 4, 9, 10, 12, 15, 16, 20];

        assert_eq!(sorted_merge(&arr1, &arr2), expected);
    }

    #[test]
    fn check_my_work2() {
        let arr1 = vec![1, 2, 5];
        let arr2 = vec![2, 3, 4, 7, 8];
        let expected = vec![1, 2, 2, 3, 4, 5, 7, 8];

        assert_eq!(sorted_merge(&arr1, &arr2), expected);
    }

    #[test]
    fn check_my_work3() {
        let arr1 = vec!["blue", "green", "red"];
        let arr2 = vec!["orange", "violet", "yellow"];
        let expected = vec!["blue", "green", "orange", "red", "violet", "yellow"];

        assert_eq!(sorted_merge(&arr1, &arr2), expected);
    }
}
