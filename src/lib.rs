//! How to use this crate
//! # Adding this as a dependency
//! ```rust, ignore
//! [dependencies]
//! wasmedge_hostfunctionexample_interface = "^1.0.0"
//! ```
//!
//! # Bringing this into scope
//! ```rust, ignore
//! use wasmedge_hostfunctionexample_interface::*;
//! ```

pub mod wasmedge_hostfunctionexample {
    // use std::os::raw::c_char;
    #[link(wasm_import_module = "insertion_sort_example")]
    extern "C" {
        pub fn insertion_sort_initialise(length: u32);
        pub fn insertion_sort_print_vector();
        pub fn insertion_sort_sort();
    }
}

pub fn initialise(length: u32) {
    unsafe {
        wasmedge_hostfunctionexample::insertion_sort_initialise(length);
    }
}

pub fn print() {
    unsafe {
        wasmedge_hostfunctionexample::insertion_sort_print_vector();
    }
}

pub fn sort() {
    unsafe {
        wasmedge_hostfunctionexample::insertion_sort_sort();
    }
}
