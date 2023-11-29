//! A module including helpers for extracting the destination register out of
//! an instruction.
use crate::registers::Register;

/// For extracting the destination register out of an instruction.
pub(super) struct Rd;

impl Rd {
    /// The bit mask for extracting the relevant bits out of the instruction.
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    /// The right shift to apply to the instruction to after extracting the
    /// relevant bits.
    const RSHIFT: usize = 7;

    /// Decode the destination register from an instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> Register {
        Register::const_from(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }

    /// Encode the destination register into an instruction.
    #[inline]
    pub(super) const fn encode(value: Register) -> u32 {
        ((value as u8) as u32) << Self::RSHIFT
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_01000_0010011);
        assert_eq!(Rd::decode(instruction), Register::S0);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0000000_00000_00000_000_01000_0000000);
        assert_eq!(Rd::encode(Register::S0), instruction);
    }
}
