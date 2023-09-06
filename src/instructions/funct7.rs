pub(super) struct Funct7;

impl Funct7 {
    /// This is useful as a reference but not actually required for extracting this part of the
    /// instruction
    #[allow(unused)]
    const MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    const RSHIFT: usize = 25;

    #[inline]
    pub(super) const fn decode(value: u32) -> u8 {
        (value >> Self::RSHIFT) as u8
    }

    #[inline]
    pub(super) const fn encode(value: u8) -> u32 {
        (value as u32) << Self::RSHIFT
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_00000_0010011);
        assert_eq!(Funct7::decode(instruction), 0b_0100100);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0100100_00000_00000_000_00000_0000000);
        assert_eq!(Funct7::encode(0b_0100100), instruction);
    }

    #[test]
    fn encode_mask() {
        let instruction = u32::from_le(0b_0110100_00000_00000_000_00000_0000000);
        assert_eq!(Funct7::encode(0b_10110100), instruction);
    }
}
