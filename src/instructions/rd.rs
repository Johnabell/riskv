use crate::registers::Register;

pub(super) struct Rd;

impl Rd {
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    const RSHIFT: usize = 7;

    #[inline]
    pub(super) const fn decode(value: u32) -> Register {
        Register::const_from(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode_u32() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_01000_0010011);
        assert_eq!(Rd::decode(instruction), Register::S0);
    }
}
