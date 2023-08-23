use std::ops::Deref;

pub(super) struct ImmI(i16);

impl ImmI {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for ImmI {
    fn from(value: u32) -> Self {
        Self((((value & Self::MASK) as i32) >> Self::RSHIFT) as i16)
    }
}

impl Deref for ImmI {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::integer::i12;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(*ImmI::from(instruction), i16::from_le(0b_0100100_01010));
    }

    #[test]
    fn from_u12_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_01000_0010011);
        assert_eq!(*ImmI::from(instruction), -1);
    }

    #[test]
    fn from_u12_min() {
        let instruction = u32::from_le(0b_1000000_00000_01100_101_01000_0010011);
        assert_eq!(*ImmI::from(instruction), i12::MIN);
    }

    #[test]
    fn from_u12_max() {
        let instruction = u32::from_le(0b_0111111_11111_01100_101_01000_0010011);
        assert_eq!(*ImmI::from(instruction), i12::MAX);
    }
}
