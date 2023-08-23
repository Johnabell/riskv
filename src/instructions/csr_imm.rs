use std::ops::Deref;

pub(super) struct CsrImm(u8);

impl CsrImm {
    const MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    const RSHIFT: usize = 15;
}

impl From<u32> for CsrImm {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u8)
    }
}

impl Deref for CsrImm {
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
        let instruction = u32::from_le(0b_0100100_01010_01100_101_01000_0010011);
        assert_eq!(*CsrImm::from(instruction), 12);
    }
}
