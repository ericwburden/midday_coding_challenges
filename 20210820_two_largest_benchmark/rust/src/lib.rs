extern crate rdxsort;
use rdxsort::*; 

pub fn find_two_max_std_sort<T>(arr: &mut [T]) -> [T; 2] where T: Ord + Copy {
    arr.sort();
    let l = arr.len();
    [arr[l - 1], arr[l - 2]]
}

pub fn find_two_max_unstable_sort<T>(arr: &mut [T]) -> [T; 2] where T: Ord + Copy {
    arr.sort_unstable();
    let l = arr.len();
    [arr[l - 1], arr[l - 2]]
}

pub fn find_two_max_rdx_sort<T>(arr: &mut [T]) -> [T; 2] where T: RdxSortTemplate + Copy {
    arr.rdxsort();
    let l = arr.len();
    [arr[l - 1], arr[l - 2]]
}

pub fn find_two_max_naive_swap<T>(arr: &mut [T]) -> [T; 2] where T: Ord + Copy + std::fmt:: Debug {
    let mut out = [arr[0], arr[0]];
    let mut temp;
    for idx in 1..arr.len() {
        let mut interim = arr[idx];
        if interim > out[0] {
            temp = out[0];
            out[0] = interim;
            interim = temp;
        }
        if interim > out[1] { out[1] = interim; }
    }
    out
}

pub fn find_two_max_mem_swap<T>(arr: &mut [T]) -> [T; 2] where T: Ord + Copy {
    let mut out = [arr[0], arr[0]];
    for idx in 1..arr.len() {
        if arr[idx] > out[0] { std::mem::swap(&mut arr[idx], &mut out[0]); }
        if arr[idx] > out[1] { out[1] = arr[idx]; }
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let test_input = vec![1, 2, 10, 8];
        let expected = [10, 8];
        assert_eq!(find_two_max_std_sort(&mut test_input.clone()), expected);
        assert_eq!(find_two_max_unstable_sort(&mut test_input.clone()), expected);
        assert_eq!(find_two_max_rdx_sort(&mut test_input.clone()), expected);
        assert_eq!(find_two_max_naive_swap(&mut test_input.clone()), expected);
        assert_eq!(find_two_max_mem_swap(&mut test_input.clone()), expected);
    }
}
