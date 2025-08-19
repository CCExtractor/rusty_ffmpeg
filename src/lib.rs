mod avutil;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes,
    unnecessary_transmutes,
    clippy::all
)]
pub mod ffi {
    pub use crate::avutil::{
        _avutil::*, channel_layout::*, common::*, error::*, pixfmt::*, rational::*,
    };
    include!(concat!(env!("OUT_DIR"), "/binding.rs"));
}
