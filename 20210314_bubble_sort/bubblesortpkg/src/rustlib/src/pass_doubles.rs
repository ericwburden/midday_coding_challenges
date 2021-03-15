use std::os::raw::{c_double, c_uint};

#[repr(C)]
pub struct DoubleSlice {
    data: *mut c_double,
    len: c_uint,
}

impl DoubleSlice {
    pub fn from_vec(v: &[f64]) -> Self {
        let mut v = v.to_vec();
        let data = v.as_mut_ptr();
        let len = v.len() as u32;
        std::mem::forget(v);

        DoubleSlice { data, len }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn to_slice(&self) -> &mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as _) }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.to_slice().to_vec()
    }
}

#[no_mangle]
pub extern "C" fn pass_one_double(c_float: c_double) -> c_double {
    println!("\nRust multiplies {} by 10", c_float);
    c_float * 10f64
}

#[no_mangle]
pub extern "C" fn pass_many_doubles(c_doubles: DoubleSlice) -> DoubleSlice {
    println!("{:?}", c_doubles.to_slice());
    let rust_double_vec: Vec<_> = c_doubles.to_slice().iter().map(|x| x * 10f64).collect();
    DoubleSlice::from_vec(&rust_double_vec)
}

#[no_mangle]
pub extern "C" fn free_double_slice(d: DoubleSlice) {
    // convert to Rust slice
    let d = d.to_slice().as_mut_ptr(); // Get pointer

    // Boxing the raw pointer gives ownership to the Box, which will properly
    // dispose of it when the Box goes out of scope.
    unsafe {
        Box::from_raw(d);
    }
}
