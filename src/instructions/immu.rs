use std::ops::Deref;

pub(super) struct ImmU(i32);

impl ImmU {
    pub(super) const MASK: u32 = u32::from_le(0b_1111111_11111_11111_111_00000_0000000);
    pub(super) const RSHIFT: usize = 12;
}

impl From<u32> for ImmU {
    fn from(value: u32) -> Self {
        Self(((value & Self::MASK) >> Self::RSHIFT) as i32)
    }
}

impl Deref for ImmU {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u32() {
        let instruction = u32::from_le(0b_0100100_01010_01100_101_11000_0110111);
        assert_eq!(
            *ImmU::from(instruction),
            i32::from_le(0b_0100100_01010_01100_101)
        );
    }
}
