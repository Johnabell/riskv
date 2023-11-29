use crate::integer::i12;

pub(super) struct SImmI;

impl SImmI {
    const U_MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    const L_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    const FULL_MASK: u32 = Self::U_MASK + Self::L_MASK;
    const U_RSHIFT: usize = 20;
    const L_RSHIFT: usize = 7;

    #[inline]
    pub(super) const fn decode(value: u32) -> i16 {
        ((((value & Self::U_MASK) as i32) >> Self::U_RSHIFT)
            + (((value & Self::L_MASK) as i32) >> Self::L_RSHIFT)) as i16
    }

    #[inline]
    pub(super) const fn encode(value: i16) -> u32 {
        let trunk = (value & i12::MASK) as u32;
        ((trunk << Self::U_RSHIFT) + (trunk << Self::L_RSHIFT)) & Self::FULL_MASK
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01010_0010011);
        assert_eq!(SImmI::decode(instruction), i16::from_le(0b_0100100_01010));
    }

    #[test]
    fn decode_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_11111_0010011);
        assert_eq!(SImmI::decode(instruction), -1);
    }

    #[test]
    fn decode_min() {
        let instruction = u32::from_le(0b_1000000_01000_01100_101_00000_0010011);
        assert_eq!(SImmI::decode(instruction), i12::MIN);
    }

    #[test]
    fn decode_max() {
        let instruction = u32::from_le(0b_0111111_11011_01100_101_11111_0010011);
        assert_eq!(SImmI::decode(instruction), i12::MAX);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0100100_00000_00000_000_01010_0000000);
        assert_eq!(SImmI::encode(i16::from_le(0b_0100100_01010)), instruction);
    }

    #[test]
    fn encode_mask() {
        let instruction = u32::from_le(0b_0100100_00000_00000_000_01010_0000000);
        assert_eq!(SImmI::encode(i16::from_le(0b_10100100_01010)), instruction);
    }

    #[test]
    fn encode_negative_one() {
        let instruction = u32::from_le(0b_1111111_00000_00000_000_11111_0000000);
        assert_eq!(SImmI::encode(-1), instruction);
    }

    #[test]
    fn encode_min() {
        let instruction = u32::from_le(0b_1000000_00000_00000_000_00000_0000000);
        assert_eq!(SImmI::encode(i12::MIN), instruction);
    }

    #[test]
    fn encode_max() {
        let instruction = u32::from_le(0b_0111111_00000_00000_000_11111_0000000);
        assert_eq!(SImmI::encode(i12::MAX), instruction);
    }
}
