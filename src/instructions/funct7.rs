use std::ops::Deref;

pub(super) struct Funct7(u8);

impl Funct7 {
    const MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    const RSHIFT: usize = 25;
}

impl From<u32> for Funct7 {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }
}

impl Deref for Funct7 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_00000_0010011);
        assert_eq!(*Funct7::from(instruction), 0b_0100100);
    }
}
