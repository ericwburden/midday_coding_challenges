#![allow(const_item_mutation, incomplete_features)]
#![feature(
    const_ptr_offset,
    result_into_ok_or_err
)]

pub fn n_smallest_numbers_sort(n: usize, numbers: &[i64]) -> Vec<i64> {
    let mut numbers = numbers.to_vec();
    numbers.sort_unstable();

    // Take the first `n` numbers and make a new Vec with them
    numbers.iter().take(n).copied().collect::<Vec<_>>()
}

pub fn n_smallest_numbers_nosort(n: usize, numbers: &[i64]) -> Vec<i64> {
    let mut buffer = Vec::with_capacity(n);
    for number in numbers {
        // If `number` is larger than the `n`th number added,
        // skip it. If the buffer has fewer than `n` numbers,
        // keep going.
        let last_v = buffer.last().unwrap_or(&i64::MAX);
        if number >= last_v && buffer.len() >= n { continue; }

        // Figure out where `number` goes via a binary search, then
        // insert it there (pushing everything else to the right).
        let idx = buffer.binary_search(number).into_ok_or_err();
        buffer.insert(idx, *number);

        // Drop any more than the first `n` values.
        buffer.truncate(n);
    }
    buffer
}

#[macro_export]
macro_rules! n_smallest_numbers_static {
    ( $n:expr, $numbers:expr ) => {{
        let mut buffer: [i64; $n] = [i64::MAX; $n];
        for number in $numbers {
            if $n == 0 || number >= buffer.last().unwrap() as &i64 { continue; }

            let mut carry = *number;
            let idx = buffer.binary_search(number).into_ok_or_err();
            for i in idx..$n {
                std::mem::swap(&mut carry, &mut buffer[i]);
            }
        }
        buffer.iter().take($n).filter(|x| *x < &i64::MAX).copied().collect::<Vec<_>>()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    type Testable = fn(usize, &[i64]) -> Vec<i64>;

    const FUNCTIONS:  [Testable; 2] = [
        n_smallest_numbers_sort,
        n_smallest_numbers_nosort,
    ];

    #[test]
    fn functions_work() {
        for fun in FUNCTIONS {
            assert_eq!(fun(3, &[100, 2, 35, 16, 1, 9, -20, 678, 2245, 44]), vec![-20, 1, 2]);
            assert_eq!(fun(0, &[1, 2, 3, 4, 5]), vec![]);
            assert_eq!(fun(5, &[4, 5]), vec![4, 5]);
            assert_eq!(fun(3, &[-1, -1, -1, 3, 5, 7]), vec![-1, -1, -1]);
            assert_eq!(fun(3, &[-1, -1, -10, -1, 3, 5, 7]), vec![-10, -1, -1]); 
        }
    }

    #[test]
    fn macro_works() {
        assert_eq!(
            n_smallest_numbers_static!(3, &[100, 2, 35, 16, 1, 9, -20, 678, 2245, 44]), 
            vec![-20, 1, 2]
        );
        assert_eq!(n_smallest_numbers_static!(0, &[1, 2, 3, 4, 5]), vec![]);
        assert_eq!(n_smallest_numbers_static!(5, &[4, 5]), vec![4, 5]);
        assert_eq!(n_smallest_numbers_static!(3, &[-1, -1, -1, 3, 5, 7]), vec![-1, -1, -1]);
        assert_eq!(n_smallest_numbers_static!(3, &[-1, -1, -10, -1, 3, 5, 7]), vec![-10, -1, -1]); 
    }
}
