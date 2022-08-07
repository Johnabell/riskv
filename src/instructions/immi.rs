use std::ops::Deref;

pub(super) struct ImmI(u16);

impl ImmI {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for ImmI {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u16)
    }
}

impl Deref for ImmI {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(*ImmI::from(instruction), u16::from_le(0b_0100100_01010));
    }
}
