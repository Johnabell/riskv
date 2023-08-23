use std::ops::Deref;

pub(super) struct Csr(u16);

impl Csr {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for Csr {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u16)
    }
}

impl Deref for Csr {
    type Target = u16;

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
        let instruction = u32::from_le(0b_1111111_11111_01100_101_11000_0110111);
        assert_eq!(*Csr::from(instruction), 4095);
    }
}
