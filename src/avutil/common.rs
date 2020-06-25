#[allow(non_snake_case)]
pub const fn MKBETAG(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (d as u32) | ((c as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
}

#[allow(non_snake_case)]
pub const fn MKTAG(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
}
