use std::ops::Deref;

use crate::Register;

pub(super) struct Rs1(Register);

impl Rs1 {
    const MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    const RSHIFT: usize = 15;
}

impl From<u32> for Rs1 {
    fn from(value: u32) -> Self {
        Self(Register::from(((value & Self::MASK) >> Self::RSHIFT) as u8))
    }
}

impl Deref for Rs1 {
    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(*Rs1::from(instruction), Register::A2);
    }
}
