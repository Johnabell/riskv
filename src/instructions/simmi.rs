use std::ops::Deref;

pub(super) struct SImmI(i16);

impl SImmI {
    const U_MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    const L_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    const U_RSHIFT: usize = 20;
    const L_RSHIFT: usize = 7;
}

impl From<u32> for SImmI {
    fn from(value: u32) -> Self {
        Self(
            ((((value & Self::U_MASK) as i32) >> Self::U_RSHIFT)
                + (((value & Self::L_MASK) as i32) >> Self::L_RSHIFT)) as i16,
        )
    }
}

impl Deref for SImmI {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::integer::i12;

    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01010_0010011);
        assert_eq!(*SImmI::from(instruction), i16::from_le(0b_0100100_01010));
    }

    #[test]
    fn from_u12_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_11111_0010011);
        assert_eq!(*SImmI::from(instruction), -1);
    }

    #[test]
    fn from_u12_min() {
        let instruction = u32::from_le(0b_1000000_01000_01100_101_00000_0010011);
        assert_eq!(*SImmI::from(instruction), i12::MIN);
    }

    #[test]
    fn from_u12_max() {
        let instruction = u32::from_le(0b_0111111_11011_01100_101_11111_0010011);
        assert_eq!(*SImmI::from(instruction), i12::MAX);
    }
}
