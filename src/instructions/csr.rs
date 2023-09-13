//! A module including helpers for extracting the `CSR` address out of a `CSR` instruction.

/// For extraction the `CSR` address out of a `CSR` instruction.
pub(super) struct Csr;

impl Csr {
    /// The bit mask for the relevant bits in the instruction.
    ///
    /// _Note_: this is provided here as a reference since it is not actually
    /// required for extracting this value from the instruction.
    #[allow(unused)]
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    /// The right shift to apply to the instruction to extract the CSR value.
    const RSHIFT: usize = 20;

    /// Decode the `CSR` address from a `CSR` instruction.
    #[inline]
    pub(super) const fn decode(value: u32) -> u16 {
        (value >> Self::RSHIFT) as u16
    }

    /// Encode the `CSR` address into a `CSR` instruction.
    #[inline]
    pub(super) const fn encode(value: u16) -> u32 {
        (value as u32) << Self::RSHIFT
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let instruction = u32::from_le(0b_1111111_11111_01100_101_11000_0110111);
        assert_eq!(Csr::decode(instruction), 0b_1111111_11111);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
        assert_eq!(Csr::encode(0b_1111111_11111), instruction);
    }
}
