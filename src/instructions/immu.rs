pub(super) struct ImmU;

impl ImmU {
    pub(super) const MASK: u32 = u32::from_le(0b_1111111_11111_11111_111_00000_0000000);
    pub(super) const RSHIFT: usize = 12;

    #[inline]
    pub(super) const fn decode(value: u32) -> i32 {
        ((value & Self::MASK) >> Self::RSHIFT) as i32
    }

    #[inline]
    pub(super) const fn encode(value: i32) -> u32 {
        (value as u32) << Self::RSHIFT
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_11000_0110111);
        assert_eq!(
            ImmU::decode(instruction),
            i32::from_le(0b_0100100_01010_01100_101)
        );
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_00000_0000000);
        assert_eq!(ImmU::encode(0b_0100100_01010_01100_101), instruction);
    }

    #[test]
    fn encode_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_11111_111_00000_0000000);
        assert_eq!(ImmU::encode(-1), instruction);
    }
}
