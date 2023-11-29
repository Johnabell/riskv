//! A module including helpers for extracting the 20-bit signed immediate value
//! out of an instruction.

/// For extracting the 20-bit signed immediate value out of an instruction.
pub(super) struct ImmU;

impl ImmU {
    /// The bit mask for the relevant bits in the instruction.
    ///
    /// _Note_: this is provided here as a reference since it is not actually
    /// required for extracting this value from the instruction.
    #[allow(unused)]
    pub(super) const MASK: u32 = u32::from_le(0b_1111111_11111_11111_111_00000_0000000);
    /// The right shift to apply to the instruction to extract the immediate value.
    pub(super) const RSHIFT: usize = 12;

    /// Decode the 20-bit signed immediate value from an instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> i32 {
        (value >> Self::RSHIFT) as i32
    }

    /// Encode the 20-bit signed immediate value into an instruction.
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
