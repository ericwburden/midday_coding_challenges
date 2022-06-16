use std::collections::HashSet;

#[allow(unused)]
fn two_number_sum(k: isize, numbers: &[isize]) -> bool {
    let mut diffs = HashSet::new();

    for number in numbers {
        if diffs.contains(number) { return true; }
        diffs.insert(k - number);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(two_number_sum(17, &[10, 15, 3, 7]));
        assert!(!two_number_sum(17, &[10, 15, 3, 8]));
        assert!(!two_number_sum(16, &[10, 15, 8, 7]));
        assert!(two_number_sum(16, &[10, 15, 8, 8]));
        assert!(!two_number_sum(32, &[]));
        assert!(!two_number_sum(32, &[32]));
        assert!(two_number_sum(32, &[32, 0]));
        assert!(!two_number_sum(-32, &[32, 0]));
        assert!(two_number_sum(-32, &[32, -64]));
    }
}

