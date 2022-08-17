use num::Num;

pub(crate) mod i12 {
    pub const MAX: i16 = 2047;
    pub const MIN: i16 = -2048;
}

pub(crate) trait AsUnsigned<N: Num + PartialOrd> {
    fn as_unsigned(self) -> N;
}

macro_rules! impl_unsigned {
    ($signed:ty, $unsigned:ty) => {
        impl AsUnsigned<$unsigned> for $signed {
            fn as_unsigned(self) -> $unsigned {
                self as $unsigned
            }
        }
        impl AsUnsigned<$unsigned> for $unsigned {
            fn as_unsigned(self) -> $unsigned {
                self as $unsigned
            }
        }
    };
}

impl_unsigned!(i8, u8);
impl_unsigned!(i16, u16);
impl_unsigned!(i32, u32);
impl_unsigned!(i64, u64);
impl_unsigned!(i128, u128);
