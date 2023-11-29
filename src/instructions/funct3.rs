pub(super) struct Funct3;

impl Funct3 {
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_111_00000_0000000);
    const RSHIFT: usize = 12;

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
        assert_eq!(Funct3::decode(instruction), 0b_101);
    }
}
