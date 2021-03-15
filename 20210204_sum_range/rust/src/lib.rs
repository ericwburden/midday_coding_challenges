/// Provides a better-looking syntax for creating a slice reference
///
/// This macro takes a number of arguments and produces a slice reference containing
/// the values of those arguments. Purely to satisfy my need for a less ugly syntax
/// for initializing slice references. Used in constructing tests.
///
/// `&[1, 2, 3, 4, 5][..]` becomes `slice![1, 2, 3, 4, 5]`
#[macro_export]
macro_rules! slice {
    ($($x:expr),+ $(,)?) => (
        &[$($x),+][..]
    );
}

/// *Sum Range:* Given an unsorted list of positive integers and a number K, return which 
/// contiguous elements of the list sum to K. 
/// 
/// If no range can be found, return NULL (or equivalent). If more than one range can sum to K, 
/// return the range that appears first. A single element may constitute a contiguous range.
///
/// Examples:
///
/// ```
/// # use rust::{sum_range, slice};
/// sum_range(&[1, 2, 3, 4, 5], 9);     // [2, 3, 4], 2 + 3 + 4 = 9
/// # assert_eq!(sum_range(&[1, 2, 3, 4, 5], 9), Some(slice![2, 3, 4]));  
///  
/// sum_range(&[7, 9, 2, 10, 8], 21);   // [9, 2, 10], 9 + 2 + 10 = 21
/// # assert_eq!(sum_range(&[7, 9, 2, 10, 8], 21), Some(slice![9, 2, 10]));
/// 
/// sum_range(&[2, 4, 6, 10, 12], 22);  // [2, 4, 6, 10], though 10 + 12 is also 22
/// # assert_eq!(sum_range(&[2, 4, 6, 10, 12], 22), Some(slice![2, 4, 6, 10]));
/// 
/// sum_range(&[5, 20, 30, 40], 30);    // [30], a single element can be a range of one
/// # assert_eq!(sum_range(&[5, 20, 30, 40], 30), Some(slice![30]));
/// 
/// sum_range(&[5, 10, 15, 20, 25], 7); // None, no way to sum to 7
/// # assert_eq!(sum_range(&[5, 10, 15, 20, 25], 7), None);
/// ```
#[rustfmt::skip]
pub fn sum_range(r: &[i32], k: i32) -> Option<&[i32]> {
    let (mut p1, mut p2) = (0usize, 0usize);
    while p2 < r.len() {
        let sum: i32 = r[p1..=p2].iter().sum();
        if sum < k { p2 += 1; }
        if sum > k { p1 += 1; }
        if sum == k { return Some(&r[p1..=p2]); }
    }
    None
}
