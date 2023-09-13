//! A module including helpers for extracting the 7-bit `Funct7` value out of
//! an instruction.

/// For extracting the 7-bit `Funct7` value out of an instruction.
pub(super) struct Funct7;

impl Funct7 {
    /// The bit mask for the relevant bits in the instruction.
    ///
    /// _Note_: this is provided here as a reference since it is not actually
    /// required for extracting this value from the instruction.
    #[allow(unused)]
    const MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    /// The right shift to apply to the instruction to extract the CSR value.
    const RSHIFT: usize = 25;

    /// Decode the 7-bit `Funct7` value from an instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> u8 {
        (value >> Self::RSHIFT) as u8
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
}
