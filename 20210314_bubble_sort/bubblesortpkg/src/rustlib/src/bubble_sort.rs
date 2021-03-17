use crate::pass_integers::{IntegerSlice};

#[no_mangle]
pub extern "C" fn bubble_sort(c_integers: IntegerSlice) -> IntegerSlice {
    let mut rust_integer_vec: Vec<_> = c_integers.to_vec();

    // The actual Rust bubble sort implementation
    let n = rust_integer_vec.len();
    for i in 0..(n-1) {
        let last_idx = n - i - 1;
        for j in 0..last_idx {
            if rust_integer_vec[j] > rust_integer_vec[j+1] {
                rust_integer_vec.swap(j, j+1);
            }
        }
    }

    IntegerSlice::from_vec(&rust_integer_vec)
}
