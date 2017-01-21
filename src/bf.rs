#[macro_export]
#[cfg(feature="use_std")]
macro_rules! __bitfield_impl_debug__ {
    ($name:ident, { $($var_name:ident),* }) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(.field(stringify!($var_name), &bf!(self.$var_name)))*
                    .finish()
            }
        }
    }
}

#[macro_export]
macro_rules! bitfield {
    ($name:ident: $ty:ty, { $($var_name:ident: $var_low:expr => $var_hi:expr),* }) => {
        #[derive(Clone, Copy, Default)]
        pub struct $name {
            val: $ty
        }

        impl $name {
            pub fn new(val: $ty) -> $name {
                $name {
                    val: val
                }
            }

            #[inline(always)]
            #[allow(dead_code)]
            pub fn raw(&self) -> $ty {
                self.val
            }

            #[inline(always)]
            #[allow(dead_code)]
            pub fn set_raw(&mut self, val: $ty) {
                self.val = val;
            }

            #[inline(always)]
            #[allow(dead_code)]
            pub fn get(&self, pos: (usize, usize)) -> $ty {
                bits!(self.val, pos.0 => pos.1)
            }

            #[inline(always)]
            #[allow(dead_code)]
            pub fn set(&mut self, pos: (usize, usize), val: $ty) {
                self.val ^= bits!(self.val, pos.0 => pos.1) << pos.0;
                self.val |= bits!(val, 0 => pos.1 - pos.0) << pos.0;
            }

            $(
                #[inline(always)]
                pub fn $var_name(&self) -> (usize, usize) {
                    ($var_low, $var_hi)
                }
            )*
        }

        __bitfield_impl_debug__!($name, { $($var_name),* });
    };
}

#[cfg(test)]
mod test {
    bitfield!(TestField: u8, {
        bottom: 0 => 5,
        top: 6 => 7
    });

    #[test]
    fn bitfield_get() {
        let test_field = TestField::new(0b10100000);
        assert_eq!(bf!(test_field.top), 0b10);
    }

    #[test]
    fn bitfield_set() {
        let mut test_field = TestField::new(0);
        bf!(test_field.top = 0b11);
        assert_eq!(test_field.raw(), 0b11000000);
    }

    #[test]
    fn bitfield_mod_recursive() {
        struct TFParent {
            tf: TestField
        }

        let mut tf_parent = TFParent { tf: TestField::new(0) };
        // Complex bitfield accesses must be surrounded by parentheses
        bf!((tf_parent.tf).top = 0b11);
        assert_eq!(tf_parent.tf.raw(), 0b11000000);
    }
}