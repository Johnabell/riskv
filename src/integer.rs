//! Traits and constants for handling integer conversion for
//! [crate::registers::Register] types.

/// The 12-bit signed integer type.
pub(crate) mod i12 {
    /// The largest value that can be represented by this integer type
    /// (2<sup>11</sup> &minus; 1).
    pub const MAX: i16 = 2047;
    /// The smallest value that can be represented by this integer type
    /// (&minus;2<sup>11</sup>).
    pub const MIN: i16 = -2048;
    /// The mask to extract an `i12` from a 16-bit integer type.
    pub const MASK: i16 = 0b_0000_1111_1111_1111;

    /// Extracts the `i12` value from the [i32] then sign-extends it to an `i16`.
    pub fn sign_extend(value: i32) -> i16 {
        if is_positive(value) {
            (value & 0x7FF) as i16
        } else {
            (value & 0xFFF | 0xF000) as i16
        }
    }

    /// Returns `true` if the provided [i32] represents a positive `i12` value.
    pub fn is_positive(value: i32) -> bool {
        value & 0b_1000_0000_0000 == 0
    }
}

/// The 21-bit signed integer type.
pub(crate) mod i21 {
    /// The largest value that can be represented by this integer type
    /// (2<sup>20</sup> &minus; 1).
    pub const MAX: i32 = 1048575;
    /// The smallest value that can be represented by this integer type
    /// (&minus;2<sup>20</sup>).
    pub const MIN: i32 = -1048576;
    /// The mask to extract an `i21` from a 16-bit integer type.
    pub const MASK: i32 = 0b_0000_0000_0001_1111_1111_1111_1111_1111;
}

/// Conversion from a signed to an unsigned type.
pub(crate) trait AsUnsigned<N> {
    /// Convert this to its associated unsigned type.
    ///
    /// Note this is a call to simply reinterpret the bytes as an unsigned type.
    /// This is not a value preserving operation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let signed = -1_i8;
    /// assert_eq!(signed.as_unsigned(), u8::MAX);
    /// let unsigned = u8::MAX;
    /// assert_eq!(unsigned.as_unsigned(), u8::MAX);
    /// ```
    fn as_unsigned(&self) -> N;
}

/// Conversion from a unsigned to an signed type.
pub(crate) trait AsSigned<N> {
    /// Convert this to its associated signed type.
    ///
    /// Note this is a call to simply reinterpret the bytes as a signed type.
    /// This is not a value preserving operation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let unsigned = u16::MAX;
    /// assert_eq!(unsigned.as_signed(), -1_i16);
    /// let signed = i16::MAX;
    /// assert_eq!(signed.as_signed(), i16::MAX);
    /// ```
    fn as_signed(&self) -> N;
}

/// Conversion to a [usize].
pub trait AsUsize {
    /// Convert this to a [usize].
    ///
    /// Note this is a call to simply reinterpret the bytes as an unsigned type.
    /// This is not a value preserving operation.
    ///
    /// # Example
    ///
    /// On a 64-bit (or narrower pointer width) system:
    ///
    /// ```ignore
    /// let signed: i64 = -1;
    /// assert_eq!(signed.as_usize(), usize::MAX);
    /// let unsigned = u64::MAX;
    /// assert_eq!(unsigned.as_usize(), usize::MAX);
    /// ```
    fn as_usize(&self) -> usize;
}

/// Implements [AsUnsigned], [AsUnsigned], and [AsUsize] for the provided
/// signed and unsigned types.
///
/// # Example
///
/// The following will implement the above mentioned traits for the built in
/// integer types
///
/// ```ignore
/// impl_signed_unsigned!(i8, u8);
/// impl_signed_unsigned!(i16, u16);
/// impl_signed_unsigned!(i32, u32);
/// impl_signed_unsigned!(i64, u64);
/// impl_signed_unsigned!(i128, u128);
/// ```
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
        impl AsUsize for $unsigned {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
        impl AsUsize for $signed {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    };
}

impl_signed_unsigned!(i8, u8);
impl_signed_unsigned!(i16, u16);
impl_signed_unsigned!(i32, u32);
impl_signed_unsigned!(i64, u64);
impl_signed_unsigned!(i128, u128);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_unsigned() {
        assert_eq!((-1_i8).as_unsigned(), u8::MAX);
        assert_eq!(u8::MAX.as_unsigned(), u8::MAX);
        assert_eq!((-1_i16).as_unsigned(), u16::MAX);
        assert_eq!(u16::MAX.as_unsigned(), u16::MAX);
        assert_eq!((-1_i32).as_unsigned(), u32::MAX);
        assert_eq!(u32::MAX.as_unsigned(), u32::MAX);
        assert_eq!((-1_i64).as_unsigned(), u64::MAX);
        assert_eq!(u64::MAX.as_unsigned(), u64::MAX);
        assert_eq!((-1_i128).as_unsigned(), u128::MAX);
        assert_eq!(u128::MAX.as_unsigned(), u128::MAX);
    }

    #[test]
    fn as_signed() {
        assert_eq!(i8::MAX.as_signed(), i8::MAX);
        assert_eq!(u8::MAX.as_signed(), -1);
        assert_eq!(i16::MAX.as_signed(), i16::MAX);
        assert_eq!(u16::MAX.as_signed(), -1);
        assert_eq!(i32::MAX.as_signed(), i32::MAX);
        assert_eq!(u32::MAX.as_signed(), -1);
        assert_eq!(i64::MAX.as_signed(), i64::MAX);
        assert_eq!(u64::MAX.as_signed(), -1);
        assert_eq!(i128::MAX.as_signed(), i128::MAX);
        assert_eq!(u128::MAX.as_signed(), -1);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn as_usize() {
        assert_eq!((-1_i32).as_usize(), usize::MAX);
        assert_eq!(u32::MAX.as_usize(), usize::MAX);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn as_usize() {
        assert_eq!((-1_i64).as_usize(), usize::MAX);
        assert_eq!(u64::MAX.as_usize(), usize::MAX);
    }

    #[test]
    fn sign_extend_i12_test() {
        let neg_1 = 0b_1111_1111_1111 as i32;
        let max_i12 = 0b_0111_1111_1111 as i32;
        let min_i12 = 0b_1000_0000_0000 as i32;
        assert_eq!(i12::sign_extend(neg_1), -1);
        assert_eq!(i12::sign_extend(max_i12), i12::MAX);
        assert_eq!(i12::sign_extend(min_i12), i12::MIN);
    }

    #[test]
    fn is_positive_i12_test() {
        let neg_1 = 0b_1111_1111_1111 as i32;
        let max_i12 = 0b_0111_1111_1111 as i32;
        assert!(!i12::is_positive(neg_1));
        assert!(i12::is_positive(max_i12));
    }
}
