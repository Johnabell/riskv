//! A module including helpers for extracting the 12-bit signed immediate value
//! out of an instruction.

/// For extracting the 12-bit signed immediate value out of an instruction.
pub(super) struct ImmI;

impl ImmI {
    /// The bit mask for the relevant bits in the instruction.
    ///
    /// _Note_: this is provided here as a reference since it is not actually
    /// required for extracting this value from the instruction.
    #[allow(unused)]
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    /// The right shift to apply to the instruction to extract the immediate value.
    const RSHIFT: usize = 20;

    /// Decode the 12-bit signed immediate value from an instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> i16 {
        ((value as i32) >> Self::RSHIFT) as i16
    }

    /// Encode the 12-bit signed immediate value into an instruction.
    #[inline]
    pub(super) const fn encode(value: i16) -> u32 {
        (value as u32) << Self::RSHIFT
    }
}

#[cfg(test)]
mod test {
    use crate::integer::i12;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i16::from_le(0b_0100100_01010));
    }

    #[test]
    fn decode_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), -1);
    }

    #[test]
    fn decode_min() {
        let instruction = u32::from_le(0b_1000000_00000_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i12::MIN);
    }

    #[test]
    fn decode_max() {
        let instruction = u32::from_le(0b_0111111_11111_01100_101_01000_0010011);
        assert_eq!(ImmI::decode(instruction), i12::MAX);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0100100_01010_00000_000_00000_0000000);
        assert_eq!(ImmI::encode(0b_0100100_01010), instruction);
    }

    #[test]
    fn encode_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
        assert_eq!(ImmI::encode(-1), instruction);
    }
}
