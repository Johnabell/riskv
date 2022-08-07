use std::ops::Deref;

pub(super) struct Funct3(u8);

impl Funct3 {
    const MASK: u32 = u32::from_le(0b_0000000_00000_00000_111_00000_0000000);
    const RSHIFT: usize = 12;
}

impl From<u32> for Funct3 {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }
}

impl Deref for Funct3 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01000_01000_101_00000_0010011);
        assert_eq!(*Funct3::from(instruction), 0b_101);
    }
}
