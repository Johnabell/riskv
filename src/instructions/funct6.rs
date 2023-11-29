//! A module including helpers for extracting the 6-bit `Funct6` value out of
//! an instruction.

/// For extracting the 6-bit `Funct6` value out of an instruction.
///
/// _Note_: On 64-bit architectures, bit 25 is part of the shift amount for
/// the shift immediate instructions. Hence rather than using `Funct7` which would
/// include bit 25, we define a `Funct6`.
pub(super) struct Funct6;

impl Funct6 {
    /// The bit mask for the relevant bits in the instruction.
    ///
    /// _Note_: this is provided here as a reference since it is not actually
    /// required for extracting this value from the instruction.
    #[allow(unused)]
    const MASK: u32 = u32::from_le(0b_1111110_00000_00000_000_00000_0000000);
    /// The right shift to apply to the instruction to extract the CSR value.
    const RSHIFT: usize = 26;

    /// Decode the 6-bit `Funct6` value from an instruction.
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
        assert_eq!(Funct6::decode(instruction), 0b_010010);
    }
}
