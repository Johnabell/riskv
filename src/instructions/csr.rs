use std::ops::Deref;

pub(super) struct CSR(u16);

impl CSR {
    const MASK: u32 = u32::from_le(0b_1111111_11111_00000_000_00000_0000000);
    const RSHIFT: usize = 20;
}

impl From<u32> for CSR {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as u16)
    }
}

impl Deref for CSR {
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
        let instruction = u32::from_le(0b_1111111_11111_01100_101_11000_0110111);
        assert_eq!(*CSR::from(instruction), 4095);
    }
}
