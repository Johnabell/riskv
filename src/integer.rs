pub(crate) mod i12 {
    pub const MAX: i16 = 2047;
    pub const MIN: i16 = -2048;
}

pub(crate) trait AsUnsigned<N> {
    fn as_unsigned(&self) -> N;
}

pub(crate) trait AsSigned<N> {
    fn as_signed(&self) -> N;
}

macro_rules! impl_signed_unsigned {
    ($signed:ty, $unsigned:ty) => {
        impl AsUnsigned<$unsigned> for $signed {
            fn as_unsigned(&self) -> $unsigned {
                *self as $unsigned
            }
        }
        impl AsUnsigned<$unsigned> for $unsigned {
            fn as_unsigned(&self) -> $unsigned {
                *self as $unsigned
            }
        }
        impl AsSigned<$signed> for $signed {
            fn as_signed(&self) -> $signed {
                *self as $signed
            }
        }
        impl AsSigned<$signed> for $unsigned {
            fn as_signed(&self) -> $signed {
                *self as $signed
            }
        }
    };
}

impl_signed_unsigned!(i8, u8);
impl_signed_unsigned!(i16, u16);
impl_signed_unsigned!(i32, u32);
impl_signed_unsigned!(i64, u64);
impl_signed_unsigned!(i128, u128);
