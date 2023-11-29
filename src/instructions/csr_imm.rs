pub(super) struct CsrImm;

impl CsrImm {
    const MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    const RSHIFT: usize = 15;

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
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(CsrImm::decode(instruction), 12);
    }
}
