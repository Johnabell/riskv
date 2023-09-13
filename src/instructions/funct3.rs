//! A module including helpers for extracting the 3-bit `Funct3` value out of
//! an instruction.

/// For extracting the 3-bit `Funct3` value out of a instruction.
pub(super) struct Funct3;

impl Funct3 {
    /// The bit mask for extracting the relevant bits out of the instruction.
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_111_00000_0000000);
    /// The right shift to apply to the instruction to after extracting the
    /// relevant bits.
    const RSHIFT: usize = 12;

    /// Decode the 3-bit `Funct3` value from a CSR instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> u8 {
        ((value & Self::MASK) >> Self::RSHIFT) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_00000_0010011);
        assert_eq!(Funct3::decode(instruction), 0b_101);
    }
}
