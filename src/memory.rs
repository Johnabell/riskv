use crate::integer::AsSigned;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Memory {
    data: Vec<u8>,
}

// Note this implementation of memory is little endian. It would be nice to make memory generic
// over endianess.
impl Memory {
    /// Get 8 bits of memory
    pub fn load_byte(&mut self, location: usize) -> i8 {
        self.resize::<1>(location);
        self.data[location].as_signed()
    }

    /// Get 16 bits of memory
    pub fn load_half(&mut self, location: usize) -> i16 {
        self.resize::<2>(location);
        i16::from_le_bytes(self.data[location..location + 2].try_into().unwrap())
    }

    /// Get 32 bits of memory
    pub fn load_word(&mut self, location: usize) -> i32 {
        self.resize::<4>(location);
        i32::from_le_bytes(self.data[location..location + 4].try_into().unwrap())
    }

    /// Set 32 bits of memory
    pub fn store_byte(&mut self, location: usize, value: i8) {
        self.resize::<1>(location);
        self.data[location..location + 1].copy_from_slice(&value.to_le_bytes());
    }

    /// Set 32 bits of memory
    pub fn store_half(&mut self, location: usize, value: i16) {
        self.resize::<2>(location);
        self.data[location..location + 2].copy_from_slice(&value.to_le_bytes());
    }

    /// Set 32 bits of memory
    pub fn store_word(&mut self, location: usize, value: i32) {
        self.resize::<4>(location);
        self.data[location..location + 4].copy_from_slice(&value.to_le_bytes());
    }

    #[inline]
    fn resize<const N: usize>(&mut self, location: usize) {
        if location + N > self.data.len() {
            self.data.resize(location + N, 0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn store_and_load_word() {
        let mut mem = Memory::default();
        mem.store_word(24, 54);
        mem.store_word(50, i32::MAX);
        mem.store_word(54, i32::MIN);
        mem.store_word(58, -1);
        assert_eq!(mem.load_word(24), 54);
        assert_eq!(mem.load_word(50), i32::MAX);
        assert_eq!(mem.load_word(54), i32::MIN);
        assert_eq!(mem.load_word(58), -1);
    }

    #[test]
    fn store_and_load_byte() {
        let mut mem = Memory::default();
        mem.store_byte(42, 54);
        mem.store_byte(43, i8::MAX);
        mem.store_byte(44, i8::MIN);
        mem.store_byte(45, -1);
        assert_eq!(mem.load_byte(42), 54);
        assert_eq!(mem.load_byte(43), i8::MAX);
        assert_eq!(mem.load_byte(44), i8::MIN);
        assert_eq!(mem.load_byte(45), -1);
    }

    #[test]
    fn store_and_load_half() {
        let mut mem = Memory::default();
        mem.store_half(68, 54);
        mem.store_half(70, i16::MAX);
        mem.store_half(72, i16::MIN);
        mem.store_half(74, -1);
        assert_eq!(mem.load_half(68), 54);
        assert_eq!(mem.load_half(70), i16::MAX);
        assert_eq!(mem.load_half(72), i16::MIN);
        assert_eq!(mem.load_half(74), -1);
    }

    #[test]
    fn store_and_load_various_sizes() {
        let mut mem = Memory::default();
        mem.store_word(0, -1);
        assert_eq!(mem.load_half(0), -1);
        assert_eq!(mem.load_half(2), -1);
        assert_eq!(mem.load_byte(0), -1);
        assert_eq!(mem.load_byte(1), -1);
        assert_eq!(mem.load_byte(2), -1);
        assert_eq!(mem.load_byte(3), -1);
    }
}
