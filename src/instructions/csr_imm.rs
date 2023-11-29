//! A module including helpers for extracting the 5-bit immediate value out of
//! a `CSR` instruction.

/// For extracting the 5-bit immediate value out of a `CSR` instruction.
pub(super) struct CsrImm;

impl CsrImm {
    /// The bit mask for extracting the relevant bits out of the instruction.
    const MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    /// The right shift to apply to the instruction to after extracting the
    /// relevant bits.
    const RSHIFT: usize = 15;

    /// Decode the 5-bit immediate value from a `CSR` instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> u8 {
        ((value & Self::MASK) >> Self::RSHIFT) as u8
    }

    /// Encode the 5-bit immediate value into a `CSR` instruction.
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
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(CsrImm::decode(instruction), 0b_01100);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0000000_00000_01100_000_00000_0000000);
        assert_eq!(CsrImm::encode(0b_01100), instruction);
    }

    #[test]
    fn encode_mask() {
        let instruction = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
        assert_eq!(CsrImm::encode(0b_0011_1111), instruction);
    }
}
