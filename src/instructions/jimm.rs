//! A module including helpers for extracting the 21-bit immediate value out of
//! an `J`-type instruction.

/// For extracting and encoding the `J` immediate from a `J`-type instruction.
///
/// The immediate bits of the `J`-type instruction are organized as follows
///
/// |     31    |   30 - 21   |    20     |   19 - 12    | 11 - 7 |   6 - 0  |
/// | --------- | ----------- | --------- | ------------ | ------ | -------- |
/// | `imm[20]` | `imm[10:1]` | `imm[11]` | `imm[19:12]` |  `rd`  | `opcode` |
/// |     1     |      10     |     1     |       8      |    5   |     7    |
pub(super) struct JImm;

impl JImm {
    /// A mask for the bits that are in the correct position.
    const FIX_POSITION_MASK: u32 = u32::from_le(0b_0000000_00000_11111_111_00000_0000000);
    /// The mask for the sign bit
    const SIGN_MASK: u32 = u32::from_le(0b_1000000_00000_00000_000_00000_0000000);
    /// The shift for the signed bit
    const SIGN_RSHIFT: usize = 11;
    /// The mask for the signed bit
    const INITIAL_BITS_MASK: u32 = u32::from_le(0b_0111111_11110_00000_000_00000_0000000);
    /// The shift for the signed bit
    const INITIAL_BITS_RSHIFT: usize = 20;
    /// The mask for the middle bit
    const MID_BIT_MASK: u32 = u32::from_le(0b_0000000_00001_00000_000_00000_0000000);
    /// The shift for the signed bit
    const MID_BIT_RSHIFT: usize = 9;

    /// The lest significant bit should be zero
    const FINAL_MASK: i32 = -2;

    /// Decode the 21-bit immediate value from an `J`-type instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> i32 {
        (((value & Self::FIX_POSITION_MASK) as i32)
            + (((value & Self::SIGN_MASK) as i32) >> Self::SIGN_RSHIFT)
            + (((value & Self::INITIAL_BITS_MASK) >> Self::INITIAL_BITS_RSHIFT) as i32)
            + (((value & Self::MID_BIT_MASK) >> Self::MID_BIT_RSHIFT) as i32))
            & Self::FINAL_MASK
    }

    /// Encode the 21-bit immediate value into an `J`-type instruction.
    #[inline]
    pub(super) const fn encode(value: i32) -> u32 {
        let value = value as u32;
        ((value << Self::SIGN_RSHIFT) & Self::SIGN_MASK)
            + ((value << Self::INITIAL_BITS_RSHIFT) & Self::INITIAL_BITS_MASK)
            + ((value << Self::MID_BIT_RSHIFT) & Self::MID_BIT_MASK)
            + (value & Self::FIX_POSITION_MASK)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::i21;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0110110_10101_10110_011_10111_0010011);
        assert_eq!(
            JImm::decode(instruction),
            i32::from_le(0b_0000000_00000_10110_011_1110110_10100)
        );
    }

    #[test]
    fn decode_all_the_ones() {
        let instruction = u32::from_le(0b_1111111_11111_11111_111_10111_0010011);
        // The expected answer is `-2 = -1 - 1` since the least significant bit is zero
        assert_eq!(JImm::decode(instruction), -2);
    }

    #[test]
    fn decode_min() {
        let instruction = u32::from_le(0b_1000000_00000_00000_000_01000_0010011);
        assert_eq!(JImm::decode(instruction), i21::MIN);
    }

    #[test]
    fn decode_max() {
        let instruction = u32::from_le(0b_0111111_11111_11111_111_11011_0010011);
        // `-1` because the least significant bit is zero
        assert_eq!(JImm::decode(instruction), i21::MAX - 1);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0110110_10101_10110_011_00000_0000000);
        assert_eq!(
            JImm::encode(i32::from_le(0b_0000000_00000_10110_011_1110110_10100)),
            instruction
        );
    }

    #[test]
    fn encode_negative_one() {
        let instruction = u32::from_le(0b_1111111_11111_11111_111_00000_0000000);
        assert_eq!(JImm::encode(-1), instruction);
    }

    #[test]
    fn encode_min() {
        let instruction = u32::from_le(0b_1000000_00000_00000_000_00000_0000000);
        assert_eq!(JImm::encode(i21::MIN), instruction);
    }

    #[test]
    fn encode_max() {
        let instruction = u32::from_le(0b_0111111_11111_11111_111_00000_0000000);
        assert_eq!(JImm::encode(i21::MAX), instruction);
    }
}
