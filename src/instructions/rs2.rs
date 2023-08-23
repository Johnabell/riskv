use std::ops::Deref;

use crate::registers::Register;

pub(super) struct Rs2(Register);

impl Rs2 {
    const MASK: u32 = u32::from_le(0b_0000000_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for Rs2 {
    fn from(value: u32) -> Self {
        Self(Register::from(((value & Self::MASK) >> Self::RSHIFT) as u8))
    }
}

impl Deref for Rs2 {
    type Target = Register;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(*Rs2::from(instruction), Register::A0);
    }
}
