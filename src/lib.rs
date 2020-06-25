#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes,
    clippy::all
)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/binding.rs"));
}

pub mod avutil;
