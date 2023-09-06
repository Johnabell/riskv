pub(super) struct Csr;

impl Csr {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;

    #[inline]
    pub(super) const fn decode(value: u32) -> u16 {
        ((value & Self::MASK) >> Self::RSHIFT) as u16
    }

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
