#![cfg_attr(not(feature="use_std"), no_std)]
extern crate bf_impl;

/// Sign extend a `size`-bit number (stored in a u32) to an i32.
/// ```
/// let i5bit = 0b11110;
/// let i32bit = bitutils::sign_extend32(i5bit, 5);
/// assert_eq!(i32bit, -2);
/// ```
#[inline]
pub fn sign_extend32(data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}

/// Extract a range of bits from a value.
/// Syntax: `bits!(val, lowbit:hibit);`
/// ```
/// #[macro_use]
/// extern crate bitutils;
/// 
/// # fn main() {
/// let bits = bits!(0b0101000u8, 3:5);
/// assert_eq!(bits, 0b101);
/// # }
/// ```
#[macro_export]
macro_rules! bits {
    ($val:expr, $low:tt : $hi:tt) => {{
        let max_bit = $crate::size_of_val(&$val) * 8 - 1;
        $val << (max_bit - $hi) >> (max_bit - $hi + $low)
    }};
}

/// Extract a bit from a value.
/// ```
/// #[macro_use]
/// extern crate bitutils;
/// 
/// # fn main() {
/// let bit = bit!(0b01000u8, 3);
/// assert_eq!(bit, 1);
/// # }
/// ```
#[macro_export]
macro_rules! bit {
    ($val:expr, $bit:expr) => { bits!($val, $bit:$bit) };
}

/******************************************************************
 * Bitfield operations
 */

#[cfg(feature="use_std")]
#[doc(hidden)]
pub use std::{
    mem::size_of_val,
    ops::{Deref, DerefMut},
};
#[cfg(not(feature="use_std"))]
#[doc(hidden)]
pub use core::{
    mem::size_of_val,
    ops::{Deref, DerefMut},
};

/// Declare a bitfield type.
/// ```
/// #[macro_use]
/// extern crate bitutils;
/// 
/// bf!(BitfieldName[u8] {
///     field1: 0:3, // lower nibble
///     field2: 4:6,
///     field3: 7:7
/// });
/// 
/// # fn main() {
/// let mut bf = BitfieldName::new(0);
/// bf.set_field3(0xF);
/// assert_eq!(bf.val, 0x80);
/// 
/// bf.val = 0xF0;
/// assert_eq!(bf.field1(), 0);
/// assert_eq!(bf.field2(), 7);
/// assert_eq!(bf.field3(), 1);
/// # }
/// ```
/// 
/// This declares a module `BitfieldName` with the members:
/// - `pub struct Bf { pub val: T, pub field1: Field1, pub field2... }`
/// - `pub fn new(val: T) -> Bf`
/// - `pub fn alias(val: &'a T) -> &'a Bf`
/// - `pub fn alias_mut(val: &'a mut T) -> &'a mut Bf`
/// 
/// Each field has the impl:
/// - `pub fn(&self) -> T`
/// - `pub set_fn(&mut self, val: T)`
/// - `pub upd_fn(&mut self, func: FnOnce(T) -> T)`
#[macro_export]
macro_rules! bf {
    ($($args:tt)*) => {
        $crate::bf_inner!($($args)*);
    };
}

#[doc(hidden)]
pub use bf_impl::bf as bf_inner;

#[cfg(test)]
mod test {
    use super::*;

    bf!(TestField[u8] {
        bottom: 0:5,
        top: 6:7,
    });

    #[test]
    fn bitfield() {
        let field = TestField::new(0b10100000);
        assert_eq!(field.top(), 0b10);
    }

    #[test]
    fn set_bitfield() {
        let mut bf = TestField::new(0);
        bf.set_top(0b11);
        assert_eq!(bf.val, 0b11000000);
    }

    #[test]
    fn upd_bitfield() {
        let mut bf = TestField::new(0);
        bf.upd_top(|x| x + 1);
        assert_eq!(bf.val, 0b01000000);
    }

    #[test]
    fn bitfield_alias() {
        let mut val = 0b10100000;
        {
            let bf = TestField::alias(&val);
            assert_eq!(bf.top(), 0b10);
        }
        let bf = TestField::alias_mut(&mut val);
        bf.set_top(0b11);
        assert_eq!(bf.val, 0b11100000);
    }

    #[test]
    fn bitfield_copyable() {
        fn takes_copy<T: Copy>(_t: T) {
        }

        takes_copy(TestField::new(0));
    }

    #[test]
    fn bitfield_formattable() {
        let out = format!("{:x?}", TestField::new(!0));
        assert_eq!(out, "TestField { bottom: 3f, top: 3 }");
    }
}
