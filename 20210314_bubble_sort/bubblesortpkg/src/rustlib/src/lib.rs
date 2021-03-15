// Import dependencies
extern crate libc;

// Modules are other .rs source files
mod pass_doubles;
mod pass_integers;
mod pass_logicals;
mod pass_strings;
mod bubble_sort;

// Export functions called by R
pub use pass_doubles::{free_double_slice, pass_many_doubles, pass_one_double};
pub use pass_integers::{free_integer_slice, pass_many_integers, pass_one_integer};
pub use pass_logicals::{free_logical_slice, pass_many_logicals, pass_one_logical};
pub use pass_strings::{free_string_slice, pass_many_strings, pass_one_string};
pub use bubble_sort::bubble_sort;
