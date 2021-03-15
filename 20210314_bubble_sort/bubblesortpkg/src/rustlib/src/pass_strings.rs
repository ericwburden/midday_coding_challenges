use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_uint};

/// Represents a String from R, an enum either wrapping a String or containing a
/// stand-in for R's NA_character_
#[derive(Clone)]
pub enum RString {
    Val(String),
    Na,
}

impl RString {
    /// Converts a plain String or string slice to an RString
    pub fn from_string(s: &str) -> Self {
        RString::Val(s.to_string())
    }

    /// Creates an RString representing NA_character_
    pub fn na() -> Self {
        RString::Na
    }

    /// Converts a C-style character pointer to an RString
    pub fn from_pointer(c: *const c_char) -> Self {
        if c.is_null() {
            RString::Na // NA_character_
        } else {
            let s = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
            RString::Val(s)
        }
    }

    /// Converts an RString to a C-Style pointer
    pub fn to_pointer(&self) -> *const c_char {
        let s = match self {
            RString::Val(s) => CString::new(s.to_string()).ok(),
            RString::Na => None,
        }; // Back to CString
        let p = match s {
            Some(ref x) => x.as_ptr(),
            None => std::ptr::null(),
        }; // Pass back a pointer
        std::mem::forget(s); // Stop Rust de-allocating the CString
        p
    }

    /// Given a closure that takes a String reference and returns a String, apply
    /// that function to the String in an RString::Val, otherwise just return the
    /// RString::Na
    pub fn transform<F: FnOnce(&String) -> String>(&self, f: F) -> Self {
        match self {
            RString::Val(s) => RString::Val(f(s)),
            _ => RString::Na,
        }
    }
}

impl fmt::Debug for RString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RString::Val(x) => f.write_fmt(format_args!("R\"{}\"", x)),
            RString::Na => f.write_str("NA_character_"),
        }
    }
}

impl fmt::Display for RString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RString::Val(x) => write!(f, "{}", x),
            RString::Na => write!(f, "NA_character_"),
        }
    }
}

/// Represents a slice containing strings, packaged by/for C
///
/// # Members
///
/// * data: *mut *const c_char - A mutable pointer to the beginning of an array of c_char pointers
/// * len: c_uint - The number of c_char pointers in the array
#[repr(C)]
pub struct StringSlice {
    data: *mut *const c_char,
    len: c_uint,
}

impl StringSlice {
    /// Returns a StringSlice given a slice of RStrings originating in Rust
    ///
    /// # Arguments
    ///
    /// * v: &[RString] - Slice of RStrings to encapsulate in StringSlice
    pub fn from_vec(v: &[RString]) -> Self {
        // Convert a slice of RStrings to pointers
        let mut return_rstr_ptrs: Vec<_> = v.iter().map(|x| x.to_pointer()).collect();
        let len = return_rstr_ptrs.len() as u32;
        let data: *mut *const c_char = return_rstr_ptrs.as_mut_ptr();

        // Need to `forget` these so they aren't de-allocated after the StringSlice
        // is passed back to C
        std::mem::forget(return_rstr_ptrs);

        StringSlice { data, len }
    }

    /// Transforms a StringSlice into a standard Vec<RString>
    pub fn to_vec(&self) -> Vec<RString> {
        let mut v = Vec::new();
        for i in 0..self.len {
            let i = i as isize;
            let c_strings_ptr: *const c_char = unsafe { *(self.data.offset(i)) };
            v.push(RString::from_pointer(c_strings_ptr));
        }
        v
    }
}

#[no_mangle]
pub extern "C" fn pass_one_string(c_string: *const c_char) -> *const c_char {
    let rust_string = RString::from_pointer(c_string); // char* pointer to RString
    let return_string = rust_string.transform(|x| x.to_uppercase()); // Modify just to prove it works
    return_string.to_pointer() // Return a C-style pointer
}

#[no_mangle]
pub extern "C" fn pass_many_strings(c_strings: StringSlice) -> StringSlice {
    // Parse C pointers into Rust Strings
    let rust_strings = c_strings.to_vec();

    println!("Rust got {:?}\n", rust_strings); // Report the contents of the StringSlice from C
    println!("--- Strings capitalized in Rust! ---\n");

    // Modify the strings and add more strings to demonstrate that the strings
    // did in fact pass through Rust and that the returned StringSlice does not
    // need to be the same length as the StringSlice received
    let mut rust_strings_upper: Vec<_> = rust_strings
        .iter()
        .map(|x| x.transform(|o| o.to_uppercase()))
        .collect();
    rust_strings_upper.push(RString::from_string("A New String"));
    rust_strings_upper.push(RString::from_string("Another New String"));
    println!("Rust sent {:?}", rust_strings_upper);

    // Parse Rust Strings back into C pointers and return
    StringSlice::from_vec(&rust_strings_upper)
}

/// Function to free the memory from a StringSlice, called from C when done with
/// a StringSlice from Rust
#[no_mangle]
pub extern "C" fn free_string_slice(s: StringSlice) {
    // convert to Rust slice
    let s = unsafe { std::slice::from_raw_parts_mut(s.data, s.len as _) };
    let s = s.as_mut_ptr(); // Get pointer

    // Boxing the raw pointer gives ownership to the Box, which will properly
    // dispose of it when the Box goes out of scope.
    unsafe {
        Box::from_raw(s);
    }
}
