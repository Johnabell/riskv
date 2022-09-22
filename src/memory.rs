use crate::integer::AsSigned;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Memory {
    data: Vec<u8>,
}

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
    fn can_create_memory_and_retrieve() {
        let mut mem = Memory::default();
        mem.store_word(67, 54);
        assert_eq!(mem.load_word(67), 54);
    }
}
