use std::os::raw::{c_int, c_uint};

// The 'logical' (boolean) values received from R are translated through C as
// int32_t -> c_int -> i32 in Rust, so the 'NA' for a logical is the same as the
// 'NA' value for an integer
const NA_LOGICAL: i32 = -2_147_483_648;

#[derive(Clone, Copy)]
pub enum RLogical {
    Val(bool),
    Na,
}

impl RLogical {
    pub fn na() -> Self {
        RLogical::Na
    }

    pub fn from_bool(b: bool) -> Self {
        RLogical::Val(b)
    }

    pub fn from_int(i: i32) -> Self {
        match i {
            1 => RLogical::Val(true),
            0 => RLogical::Val(false),
            NA_LOGICAL => RLogical::Na,
            _ => panic!("{} does not represent a valid boolean value", i),
        }
    }

    pub fn to_int(&self) -> c_int {
        match &self {
            RLogical::Val(true) => 1,
            RLogical::Val(false) => 0,
            RLogical::Na => NA_LOGICAL,
        }
    }
}

#[repr(C)]
pub struct LogicalSlice {
    data: *mut c_int,
    len: c_uint,
}

impl LogicalSlice {
    pub fn from_vec(v: &[RLogical]) -> Self {
        let mut v: Vec<_> = v.iter().map(|x| x.to_int()).collect();
        let data = v.as_mut_ptr();
        let len = v.len() as u32;
        std::mem::forget(v);

        LogicalSlice { data, len }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn to_slice(&self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as _) }
    }

    pub fn to_vec(&self) -> Vec<RLogical> {
        self.to_slice()
            .iter()
            .map(|x| RLogical::from_int(*x))
            .collect()
    }
}

#[no_mangle]
pub extern "C" fn pass_one_logical(c_logical: c_int) -> c_int {
    let logical = RLogical::from_int(c_logical);
    let result = match logical {
        RLogical::Val(x) => {
            println!("Rust toggles {} to {}", x, !x);
            RLogical::Val(!x)
        }
        RLogical::Na => {
            println!("Rust received 'NA' value");
            RLogical::Na
        }
    };
    result.to_int()
}

#[no_mangle]
pub extern "C" fn pass_many_logicals(c_logicals: LogicalSlice) -> LogicalSlice {
    // Cast the LogicalSlice to a Vec<RLogical>, flip all the booleans, collect the result
    let rust_logical_vec: Vec<_> = c_logicals
        .to_vec()
        .into_iter()
        .map(|x| match x {
            RLogical::Val(x) => RLogical::Val(!x),
            RLogical::Na => RLogical::Na,
        })
        .collect();

    LogicalSlice::from_vec(&rust_logical_vec)
}

#[no_mangle]
pub extern "C" fn free_logical_slice(l: LogicalSlice) {
    // convert to Rust slice
    let l = l.to_slice().as_mut_ptr(); // Get pointer

    // Boxing the raw pointer gives ownership to the Box, which will properly
    // dispose of it when the Box goes out of scope.
    unsafe {
        Box::from_raw(l);
    }
}
