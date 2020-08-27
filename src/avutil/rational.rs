/// Bindgen is unable to generate static inline functions since they don't exist
/// in linked library. So we need this.
/// Ref: https://github.com/rust-lang/rust-bindgen/issues/1344
use crate::ffi::AVRational;

pub fn av_make_q(num: libc::c_int, den: libc::c_int) -> AVRational {
    AVRational { num, den }
}

pub fn av_cmp_q(a: AVRational, b: AVRational) -> libc::c_int {
    let tmp = i64::from(a.num) * i64::from(b.den) - i64::from(b.num) * i64::from(a.den);

    if tmp != 0 {
        (((tmp ^ i64::from(a.den) ^ i64::from(b.den)) >> 63) | 1) as libc::c_int
    } else if b.den != 0 && a.den != 0 {
        0
    } else if a.num != 0 && b.num != 0 {
        (a.num >> 31) - (b.num >> 31)
    } else {
        libc::c_int::MIN
    }
}

pub fn av_q2d(a: AVRational) -> libc::c_double {
    libc::c_double::from(a.num) / libc::c_double::from(a.den)
}

pub fn av_inv_q(q: AVRational) -> AVRational {
    AVRational {
        num: q.den,
        den: q.num,
    }
}
