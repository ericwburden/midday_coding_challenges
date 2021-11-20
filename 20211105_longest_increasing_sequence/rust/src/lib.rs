
// #![feature(result_into_ok_or_err)]
// fn longest_increasing_sequence(numbers: &[usize]) -> usize {
//     // Create a zero-fileld vector of the same length as the input and create a
//     // "pointer" to the first zero value in `subsequence`. This also indicates the
//     // length of the longest subsequence found.
//     let mut subsequence = vec![0; numbers.len()];
//     let mut len = 0;

//     // For each number in the input (after the first), insert that number into the 
//     // subsequence in whatever position allows the subsequence to retain its order.
//     // When inserting after the end (because the number is larger than any number
//     // currently in `subsequence`), move the "pointer" forward by one. Because the
//     // non-zero numbers in `subsequence` are sorted, you can find the target position
//     // using a binary search.
//     for number in numbers {
//         let insertion_point = subsequence[0..len]
//             .binary_search(number)
//             .into_ok_or_err();
//         if subsequence[insertion_point] == 0 { len += 1; }
//         subsequence[insertion_point] = *number;

//         println!("{:>2?}", numbers);
//         println!("{:>2?}", subsequence);
//         println!();
//     }

//     len
// }

fn longest_increasing_sequence(numbers: &[usize]) -> usize {
    let mut subsequence = vec![0; numbers.len()];
    let mut len = 0;

    for number in numbers {
        if number <= &subsequence[0] {
            subsequence[0] = *number;
            continue;
        }
        if len == 0 || number > &subsequence[len - 1] {
            subsequence[len] = *number;
            len += 1;
            continue;
        }
        match subsequence[0..len].binary_search(number) {
            Ok(_) => continue,
            Err(idx) => subsequence[idx] = *number,
        }

        println!("{:>2?}", numbers);
        println!("{:>2?}", subsequence);
        println!();
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_correctly_identify_lis_for_examples_with_unique_values() {
        let test_cases = [
        	(vec![1], 1),
        	(vec![9, 6, 4, 2], 1),
            (vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 1),
            (vec![10, 5, 6, 7, 3, 4, 8, 9], 5),
        	(vec![1, 2, 3, 2], 3),
        	(vec![10, 22, 9, 33, 21, 50, 41, 60, 68, 90], 7),
        	(vec![7, 10, 3, 5, 1, 6, 11, 21, 9], 5),
        	(vec![10, 12, 4, 6, 100, 2, 56, 34, 79, 45, 33, 67, 89], 6),
        	(vec![10, 12, 4, 6, 100, 2, 56, 34, 79], 4),
        	(vec![100, 200, 300, 400, 500, 10, 20, 30, 40, 50, 60, 1, 2, 3, 4, 5, 6, 7, 1000], 8),
        ];

        for (input, expected) in test_cases {
            assert_eq!(longest_increasing_sequence(&input), expected);
        }
    }

    
    #[test]
    fn testing_for_time_complexity() {
        let test_cases = [
        	((0..100).rev().collect::<Vec<usize>>(), 1),
        	((0..1_000).rev().collect::<Vec<usize>>(), 1),
        	((0..10_000).rev().collect::<Vec<usize>>(), 1),
        ];

        for (input, expected) in test_cases {
            assert_eq!(longest_increasing_sequence(&input), expected);
        }
    }
    
}
