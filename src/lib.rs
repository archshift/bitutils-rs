#![cfg_attr(not(feature="use_std"), no_std)]

#[inline]
pub fn sign_extend32(data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}

#[macro_export]
macro_rules! bits {
    ($val:expr, $low:tt : $hi:tt) => {{
        let max_bit = $crate::size_of_val(&$val) * 8 - 1;
        $val << (max_bit - $hi) >> (max_bit - $hi + $low)
    }};
}

#[macro_export]
macro_rules! bit {
    ($val:expr, $bit:expr) => { bits!($val, $bit:$bit) };
}

/******************************************************************
 * Bitfield operations
 */

#[cfg(feature="use_std")]
pub use std::{
    mem::size_of_val,
    ops::{Deref, DerefMut},
};
#[cfg(not(feature="use_std"))]
pub use core::{
    mem::size_of_val,
    ops::{Deref, DerefMut},
};

#[macro_export]
#[cfg(feature="use_std")]
macro_rules! __bitfield_impl_debug__ {
    ($name:ident, { $($var_name:ident),* }) => {
        impl ::std::fmt::Debug for Bf {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(.field(stringify!($var_name), &self.$var_name.get()))*
                    .finish()
            }
        }
    }
}

#[macro_export]
#[cfg(not(feature="use_std"))]
macro_rules! __bitfield_impl_debug__ {
    ($name:ident, { $($var_name:ident),* }) => {}
}

#[macro_export]
macro_rules! bf {
    ($name:ident [$ty:ty] { $($var_name:ident: $var_low:tt : $var_hi:tt),* $(,)* }) => {
        #[allow(non_snake_case)]
        pub mod $name {
            $(
                #[repr(C)]
                pub struct $var_name {
                    _dont_instantiate: ()
                }

                #[allow(dead_code)]
                impl $var_name {
                    #[inline(always)]
                    pub fn get(&self) -> $ty {
                        let bfptr = self as *const Self as *const Bf;
                        let _ = self;
                        let val = unsafe { (*bfptr).val };
                        bits!(val, $var_low : $var_hi)
                    }

                    #[inline(always)]
                    pub fn set(&mut self, new: $ty) {
                        let bfptr = self as *mut Self as *mut Bf;
                        let _ = self;
                        let val = unsafe { &mut (*bfptr).val };
                        *val ^= bits!(*val, $var_low : $var_hi) << $var_low;
                        *val |= bits!(new, 0 : ($var_hi - $var_low)) << $var_low;
                    }

                    #[inline(always)]
                    pub fn update<F>(&mut self, func: F)
                        where F: FnOnce($ty) -> $ty {
                        let old = self.get();
                        self.set(func(old))
                    }
                }
            )*

            #[repr(C)]
            pub struct Fields {
                $( pub $var_name: $var_name ),*
            }

            #[repr(transparent)]
            #[derive(Copy, Clone)]
            pub struct Bf {
                pub val: $ty,
            }
            impl Bf {
                #[inline(always)]
                pub fn new(val: $ty) -> Self {
                    Self {
                        val: val
                    }
                }
            }
            impl $crate::Deref for Bf {
                type Target = Fields;
                #[inline(always)]
                fn deref(&self) -> &Fields {
                   unsafe { &*(self as *const Self as *const Fields) } 
                }
            }
            impl $crate::DerefMut for Bf {
                #[inline(always)]
                fn deref_mut(&mut self) -> &mut Fields {
                   unsafe { &mut *(self as *mut Self as *mut Fields) } 
                }
            }

            __bitfield_impl_debug__!($name, { $($var_name),* });

            #[allow(dead_code)]
            #[inline(always)]
            pub fn new(val: $ty) -> Bf {
                Bf::new(val)
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn alias<'a>(val: &'a $ty) -> &'a Bf {
                unsafe { &*(val as *const $ty as *const Bf) }
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn alias_mut<'a>(val: &'a mut $ty) -> &'a mut Bf {
                unsafe { &mut *(val as *mut $ty as *mut Bf) }
            }
        }
    };
}

#[cfg(test)]
mod test {
    bf!(TestField[u8] {
        bottom: 0:5,
        top: 6:7,
    });

    #[test]
    fn bitfield_get() {
        let field = TestField::new(0b10100000);
        assert_eq!(field.top.get(), 0b10);
    }

    #[test]
    fn bitfield_set() {
        let mut bf = TestField::new(0);
        bf.top.set(0b11);
        assert_eq!(bf.val, 0b11000000);
    }

    #[test]
    fn bitfield_update() {
        let mut bf = TestField::new(0);
        bf.top.update(|x| x + 1);
        assert_eq!(bf.val, 0b01000000);
    }

    #[test]
    fn bitfield_alias() {
        let mut val = 0b10100000;
        {
            let bf = TestField::alias(&val);
            assert_eq!(bf.top.get(), 0b10);
        }
        let bf = TestField::alias_mut(&mut val);
        bf.top.set(0b11);
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
