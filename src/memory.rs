#[derive(Debug, Default, PartialEq, Eq)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Get 32 bits of memory
    pub fn load_word(&mut self, location: usize) -> i32 {
        if location + 4 > self.data.len() {
            self.data.resize(location + 4, 0);
        }
        i32::from_le_bytes(self.data[location..location + 4].try_into().unwrap())
    }

    /// Set 32 bits of memory
    pub fn store_word(&mut self, location: usize, value: i32) {
        if location + 4 > self.data.len() {
            self.data.resize(location + 4, 0);
        }
        self.data[location..location + 4].copy_from_slice(&value.to_le_bytes());
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
