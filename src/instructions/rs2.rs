use crate::registers::Register;

pub(super) struct Rs2;

impl Rs2 {
    const MASK: u32 = u32::from_le(0b_0000000_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;

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
        assert_eq!(Rs2::decode(instruction), Register::A0);
    }

    #[test]
    fn encode() {
        let instruction = u32::from_le(0b_0000000_01010_00000_000_00000_0000000);
        assert_eq!(Rs2::encode(Register::A0), instruction);
    }
}
