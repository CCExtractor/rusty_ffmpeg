/// Bindgen is unable to generate static inline functions since they don't exist
/// in linked library. So we need this.
/// Ref: https://github.com/rust-lang/rust-bindgen/issues/1344
use crate::ffi::AVRational;

/// Create an AVRational.
///
/// Useful for compilers that do not support compound literals.
///
/// @note The return value is not reduced.
/// @see av_reduce()
pub fn av_make_q(num: libc::c_int, den: libc::c_int) -> AVRational {
    AVRational { num, den }
}

/// Compare two rationals.
///
/// @param a First rational
/// @param b Second rational
///
/// @return One of the following values:
///         - 0 if `a == b`
///         - 1 if `a > b`
///         - -1 if `a < b`
///         - `INT_MIN` if one of the values is of the form `0 / 0`
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

/// Convert an AVRational to a `double`.
/// @param a AVRational to convert
/// @return `a` in floating-point form
/// @see av_d2q()
pub fn av_q2d(a: AVRational) -> libc::c_double {
    libc::c_double::from(a.num) / libc::c_double::from(a.den)
}

/// Invert a rational.
/// @param q value
/// @return 1 / q
pub fn av_inv_q(q: AVRational) -> AVRational {
    AVRational {
        num: q.den,
        den: q.num,
    }
}
