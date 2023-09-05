pub(super) struct ImmI;

impl ImmI {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;

    #[inline]
    pub(super) const fn decode(value: u32) -> i16 {
        (((value & Self::MASK) as i32) >> Self::RSHIFT) as i16
    }
}

#[cfg(test)]
mod test {
    use crate::integer::i12;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn decode_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i16::from_le(0b_0100100_01010));
    }

    #[test]
    fn decode_u32_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), -1);
    }

    #[test]
    fn decode_u32_min() {
        let instruction = u32::from_le(0b_1000000_00000_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i12::MIN);
    }

    #[test]
    fn decode_u32_max() {
        let instruction = u32::from_le(0b_0111111_11111_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i12::MAX);
    }
}
