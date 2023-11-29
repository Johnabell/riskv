pub(super) struct Shamt;

impl Shamt {
    const MASK: u32 = u32::from_le(0b_0000001_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;

    #[inline]
    pub(super) const fn decode(value: u32) -> u8 {
        ((value & Self::MASK) >> Self::RSHIFT) as u8
    }

    #[inline]
    pub(super) const fn encode(value: u8) -> u32 {
        (value as u32) << Self::RSHIFT & Self::MASK
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_00110_01100_101_01000_0010011);
        assert_eq!(Shamt::decode(instruction), 6);
        // For 64 bit architectures, bit 25 is interpreted as part of the shift amount
        let instruction = u32::from_le(0b_0100101_00110_01100_101_01000_0010011);
        assert_eq!(Shamt::decode(instruction), 38);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0000000_00110_00000_000_00000_0000000);
        assert_eq!(Shamt::encode(6), instruction);
        // For 64 bit architectures, bit 25 is interpreted as part of the shift amount
        let instruction = u32::from_le(0b_0000001_00110_00000_000_00000_0000000);
        assert_eq!(Shamt::encode(38), instruction);
    }
}
