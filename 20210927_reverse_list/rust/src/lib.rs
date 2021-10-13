/// Reverse a mutable list in place
pub fn reverse<T: Copy>(input: &mut Vec<T>) {
    let mut p1 = 0;
    let mut p2 = input.len() - 1;

    while p1 < p2 {
        input.swap(p1, p2);
        p1 += 1;
        p2 -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an_even_length_list() {
        let mut input = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![6, 5, 4, 3, 2, 1];
        reverse(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_an_odd_length_list() {
        let mut input = vec![1, 2, 3, 4, 5];
        let expected = vec![5, 4, 3, 2, 1];
        reverse(&mut input);
        assert_eq!(input, expected);
    }
}
