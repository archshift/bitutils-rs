#![cfg_attr(not(feature="use_std"), no_std)]

#[inline]
pub fn sign_extend(data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}

#[macro_export]
#[cfg(feature="use_std")]
macro_rules! bits {
    ($val:expr, $low:expr => $hi:expr) => {{
        let max_bit = ::std::mem::size_of_val(&$val) * 8 - 1;
        $val << (max_bit - $hi) >> (max_bit - $hi + $low)
    }};
}
#[macro_export]
#[cfg(not(feature="use_std"))]
macro_rules! bits {
    ($val:expr, $low:expr => $hi:expr) => {{
        let max_bit = ::core::mem::size_of_val(&$val) * 8 - 1;
        $val << (max_bit - $hi) >> (max_bit - $hi + $low)
    }};
}

#[macro_export]
macro_rules! bit {
    ($val:expr, $bit:expr) => { bits!($val, $bit => $bit) };
}

#[macro_use]
pub mod bfdesc;
#[macro_use]
pub mod bf;