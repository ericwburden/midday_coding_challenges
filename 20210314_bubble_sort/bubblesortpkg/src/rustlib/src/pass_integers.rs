use std::os::raw::{c_int, c_uint};

pub const NA_INTEGER: i32 = -2_147_483_648; // The value R uses to represent NA_INTEGER

#[repr(C)]
pub struct IntegerSlice {
    data: *mut c_int,
    len: c_uint,
}

impl IntegerSlice {
    pub fn from_vec(v: &[i32]) -> Self {
        let mut v = v.to_vec();
        let data = v.as_mut_ptr();
        let len = v.len() as u32;
        std::mem::forget(v);

        IntegerSlice { data, len }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn to_slice(&self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as _) }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.to_slice().to_vec()
    }
}

#[no_mangle]
pub extern "C" fn pass_one_integer(c_integer: c_int) -> c_int {
    if c_integer == NA_INTEGER {
        println!("Rust received 'NA' value");
        return c_integer;
    }
    println!("Rust multiplies {} by 10", c_integer);
    match c_integer.checked_mul(10) {
        Some(n) => n,
        None => NA_INTEGER,
    }
}

#[no_mangle]
pub extern "C" fn pass_many_integers(c_integers: IntegerSlice) -> IntegerSlice {
    let rust_integer_vec: Vec<_> = c_integers
        .to_slice()
        .iter()
        .map(|x| match x.checked_mul(10) {
            Some(n) => n,
            None => NA_INTEGER,
        })
        .collect();
    IntegerSlice::from_vec(&rust_integer_vec)
}

#[no_mangle]
pub extern "C" fn free_integer_slice(i: IntegerSlice) {
    // convert to Rust slice
    let i = i.to_slice().as_mut_ptr(); // Get pointer

    // Boxing the raw pointer gives ownership to the Box, which will properly
    // dispose of it when the Box goes out of scope.
    unsafe {
        Box::from_raw(i);
    }
}
