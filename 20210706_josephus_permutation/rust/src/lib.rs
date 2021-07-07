fn josephus<T:Clone+Copy>(xs:Vec<T>,k:usize)-> Vec<T> {
    let mut in_vec = xs.to_owned();
    let mut out_vec = Vec::with_capacity(xs.len());
    let mut curr_idx = 0;

    while !in_vec.is_empty() {
        curr_idx = (curr_idx + k - 1) % in_vec.len();
        out_vec.push(in_vec.remove(curr_idx));
    }

    out_vec
}


#[cfg(test)]
mod josephus {
    use super::*;

    #[test]
    fn test_works_with_integers() {
      assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
      assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]);
    }
    #[test]
    fn test_works_with_strings() {
        assert_eq!(josephus("CodeWars".chars().collect::<Vec<char>>(), 4), "esWoCdra".chars().collect::<Vec<char>>());
    }
}
