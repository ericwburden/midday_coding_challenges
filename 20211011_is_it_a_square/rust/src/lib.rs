pub fn is_sqrt_robust(n: i64) -> bool {
    if n < 0 { return false; }
    let max_search_range = (n / 2) + 1;
    let search_range = 0..=max_search_range;
    let found_square = search_range.collect::<Vec<_>>().binary_search_by(|s| (s*s).cmp(&n));
    found_square.is_ok()
}

pub fn is_sqrt_naive(n: i64) -> bool {
    if n < 0 { return false; }
    // Take the square root, square it, and check if it equals the input
    ((n as f64).sqrt() as i64).pow(2) == n 
}

use std::convert::{From, TryFrom};

fn is_sqrt_safe(n: i64) -> bool {
    if n < 0 { return false; }
    let square_root_of_n = match f64::try_from(n) {
        Err(_) => return false,
        Ok(x) => x.sqrt(),
    };
    match i64::try_from(square_root_of_n) {
        Err(_) => return false,
        Ok(x) => x*x == n,
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn should_find_example_squares_with_both_methods() {
        let test_cases = [(-1, false), (0, true),  (3, false),
                          (4, true),   (25, true), (26, false)];
        for case in &test_cases {
            assert_eq!(is_sqrt_naive(case.0), case.1);
            assert_eq!(is_sqrt_robust(case.0), case.1);
        }
    }
}
