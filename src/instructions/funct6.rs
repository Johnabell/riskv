/// On 64-bit architectures, bit 25 is part of the shift amount - hence rather than having a funct7,
/// we have a funct6
pub(super) struct Funct6;

impl Funct6 {
    const MASK: u32 = u32::from_le(0b_1111110_00000_00000_000_00000_0000000);
    const RSHIFT: usize = 26;

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
    fn decode_u32() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_00000_0010011);
        assert_eq!(Funct6::decode(instruction), 0b_010010);
    }
}
