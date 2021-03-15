use crate::pass_integers::{IntegerSlice};

#[no_mangle]
pub extern "C" fn bubble_sort(c_integers: IntegerSlice) -> IntegerSlice {
    let mut rust_integer_vec: Vec<_> = c_integers.to_slice().to_vec();

    // The actual Rust bubble sort implementation
    let mut idx = 0;
    let last_idx = rust_integer_vec.len() - 2;
    let mut swapped = false;
    loop {
        while idx <= last_idx {
            if rust_integer_vec[idx] > rust_integer_vec[idx + 1] {
                rust_integer_vec.swap(idx, idx + 1);
                swapped = true;
            }
            idx += 1;
        }
        if swapped {
            idx = 0;
            swapped = false;
            continue;
        }
        break; // If execution gets here, the sorting is done
    }

    IntegerSlice::from_vec(&rust_integer_vec)
}
