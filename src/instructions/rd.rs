use std::ops::Deref;

use crate::registers::Register;

pub(super) struct Rd(Register);

impl Rd {
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    const RSHIFT: usize = 7;
}

impl From<u32> for Rd {
    fn from(value: u32) -> Self {
        Self(Register::from(((value & Self::MASK) >> Self::RSHIFT) as u8))
    }
}

impl Deref for Rd {
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
        let instruction = u32::from_le(0b_0100100_01000_01000_101_01000_0010011);
        assert_eq!(*Rd::from(instruction), Register::S0);
    }
}
