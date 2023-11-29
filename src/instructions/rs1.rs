use crate::registers::Register;

pub(super) struct Rs1;

impl Rs1 {
    const MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    const RSHIFT: usize = 15;

    #[inline]
    pub(super) const fn decode(value: u32) -> Register {
        Register::const_from(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }

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
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(Rs1::decode(instruction), Register::A2);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0000000_00000_01100_000_00000_0000000);
        assert_eq!(Rs1::encode(Register::A2), instruction);
    }
}
