//! A module including helpers for extracting the `13`-bit immediate value out of
//! an `B`-type instruction.

/// For extracting and encoding the `B` immediate from a `B`-type instruction.
///
/// The immediate bits of the `B`-type instruction are organized as follows
///
/// |     31    |   30 - 25   | 24 - 20 | 19 - 15 | 14 - 12 |   11 - 8   |     7     |   6 - 0  |
/// | --------- | ----------- | ------- | ------- | ------- | ---------- | --------- | -------- |
/// | `imm[12]` | `imm[10:5]` |  `rs2`  |  `rs1`  | `func3` | `imm[4:1]` | `imm[11]` | `opcode` |
/// |     1     |      6      |    5    |    5    |    3    |      4     |     1     |     7    |
pub(super) struct BImm;

impl BImm {
    /// The mask for the sign bit
    const SIGN_MASK: u32 = u32::from_le(0b_1000000_00000_00000_000_00000_0000000);
    /// The shift for the signed bit
    const SIGN_RSHIFT: usize = 19;
    /// The mask for the middle bit
    const LOW_BIT_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_00001_0000000);
    /// The shift for the signed bit
    const LOW_BIT_LSHIFT: usize = 4;
    /// The mask for the sign bit
    const UPPER_MASK: u32 = u32::from_le(0b_0111111_00000_00000_000_00000_0000000);
    /// The shift for the signed bit
    const UPPER_RSHIFT: usize = 20;
    /// The mask for the sign bit
    const LOW_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11110_0000000);
    /// The shift for the signed bit
    const LOW_RSHIFT: usize = 7;
    /// The lest significant bit should be zero
    const FINAL_MASK: i16 = -2;

    /// Decode the 13-bit immediate value from an `B`-type instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> i16 {
        ((((value & Self::SIGN_MASK) as i32) >> Self::SIGN_RSHIFT)
            + (((value & Self::LOW_BIT_MASK) as i32) << Self::LOW_BIT_LSHIFT)
            + (((value & Self::UPPER_MASK) as i32) >> Self::UPPER_RSHIFT)
            + (((value & Self::LOW_MASK) as i32) >> Self::LOW_RSHIFT)) as i16
            & Self::FINAL_MASK
    }

    /// Encode the 13-bit immediate value into an `B`-type instruction.
    #[inline]
    pub(super) const fn encode(value: i16) -> u32 {
        let value = value as u32;
        ((value << Self::SIGN_RSHIFT) & Self::SIGN_MASK)
            + ((value >> Self::LOW_BIT_LSHIFT) & Self::LOW_BIT_MASK)
            + ((value << Self::UPPER_RSHIFT) & Self::UPPER_MASK)
            + ((value << Self::LOW_RSHIFT) & Self::LOW_MASK)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::i13;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0110110_10101_10110_011_10111_0010011);
        assert_eq!(
            BImm::decode(instruction),
            i16::from_le(0b_000_0_1_110110_10110)
        );
    }

    #[test]
    fn decode_negative_one() {
        // This is not correct yet
        let instruction = u32::from_le(0b_1111111_11011_01100_101_11111_0010011);
        // The expected answer is `-2 = -1 - 1` since the least significant bit is zero
        assert_eq!(BImm::decode(instruction), -2);
    }

    #[test]
    fn decode_min() {
        // This is not correct yet
        let instruction = u32::from_le(0b_1000000_00100_01100_101_00000_0010011);
        assert_eq!(BImm::decode(instruction), i13::MIN);
    }

    #[test]
    fn decode_max() {
        // This is not correct yet
        let instruction = u32::from_le(0b_0111111_11011_01100_101_11111_0010011);
        // `-1` because the least significant bit is zero
        assert_eq!(BImm::decode(instruction), i13::MAX - 1);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0110110_00000_00000_000_10101_0000000);
        assert_eq!(
            BImm::encode(i16::from_le(0b_000_0_1_110110_10100)),
            instruction
        );
    }

    #[test]
    fn encode_negative_one() {
        let instruction = u32::from_le(0b_1111111_00000_00000_000_11111_0000000);
        assert_eq!(BImm::encode(-1), instruction);
    }

    #[test]
    fn encode_min() {
        let instruction = u32::from_le(0b_1000000_00000_00000_000_00000_0000000);
        assert_eq!(BImm::encode(i13::MIN), instruction);
    }

    #[test]
    fn encode_max() {
        let instruction = u32::from_le(0b_0111111_00000_00000_000_11111_0000000);
        assert_eq!(BImm::encode(i13::MAX), instruction);
    }
}
