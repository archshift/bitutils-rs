#[macro_export]
macro_rules! bits {
    ($val:expr, $low:expr => $hi:expr) => {{
        let max_bit = ::std::mem::size_of_val(&$val) * 8 - 1;
        $val << (max_bit - $hi) >> (max_bit - $hi + $low)
    }};
}

#[cfg(feature="use_std")]
use std::ops::{BitOrAssign, BitXorAssign, Shl, Shr};
#[cfg(feature="use_std")]
use std::marker::PhantomData;

#[cfg(not(feature="use_std"))]
use core::ops::{BitOrAssign, BitXorAssign, Shl, Shr};
#[cfg(not(feature="use_std"))]
use core::marker::PhantomData;

#[allow(dead_code)]
pub struct BfPos<T>
    where T: Copy + BitOrAssign + BitXorAssign
                  + Shl<usize, Output=T> + Shr<usize, Output=T> {
    pos: (usize, usize),
    marker: PhantomData<T>
}
impl<T> BfPos<T>
    where T: Copy + BitOrAssign + BitXorAssign
                  + Shl<usize, Output=T> + Shr<usize, Output=T> {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn new(pos: (usize, usize)) -> BfPos<T> {
        BfPos::<T> {
            pos: pos,
            marker: PhantomData
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn extract(&self, val: T) -> T {
        let pos = self.pos;
        bits!(val, pos.0 => pos.1)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn place(&self, mut val: T, new_val: T) -> T {
        let pos = self.pos;
        val ^= bits!(val, pos.0 => pos.1) << pos.0;
        val |= bits!(new_val, 0 => pos.1 - pos.0) << pos.0;
        val
    }
}


#[macro_export]
macro_rules! bfdesc {
    ($name:ident: $ty:ty, { $($var_name:ident: $var_low:expr => $var_hi:expr),* }) => {
        #[allow(non_snake_case)]
        pub mod $name {
            $(
                #[inline(always)]
                pub fn $var_name() -> $crate::bfdesc::BfPos<$ty> {
                    $crate::bfdesc::BfPos::<$ty>::new(($var_low, $var_hi))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! bf {
    {$var:tt.$item:ident} => ($var.get($var.$item()));
    {$var:tt.$item:ident = $val:expr} => ({
        let pos = $var.$item();
        $var.set(pos, $val)
    });
    {$var:tt @ $item:path} => (
        $item().extract($var)
    );
    {$var:tt @ $item:path = $val:expr} => (
        $var = $item().place($var, $val)
    );
    {$var:tt @ $item:path as $val:expr} => (
        $item().place($var, $val)
    );
}

#[cfg(test)]
mod test {
    const MAGIC: u8 = 0b10101010;

    mod m {
        bfdesc!(TestDesc: u8, {
            bottom: 0 => 5
        });
    }

    #[test]
    pub fn bfdesc_extract() {
        assert_eq!(bf!(MAGIC @ m::TestDesc::bottom), 0b101010);
    }

    #[test]
    pub fn bfdesc_place() {
        let mut magic = MAGIC;
        bf!(magic @ m::TestDesc::bottom = 0b110111);
        assert_eq!(magic, 0b10110111);
    }

    #[test]
    pub fn bfdesc_as() {
        assert_eq!(bf!(MAGIC @ m::TestDesc::bottom as 0b110111), 0b10110111);
    }
}