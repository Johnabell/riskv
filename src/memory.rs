use std::marker::PhantomData;

use num::Zero;

use crate::integer::AsIndex;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Memory<Signed, Unsigned> {
    data: Vec<Signed>, // TODO: consider Vec<Option<Signed>> for tracking uninitialised memory
    _marker2: PhantomData<Unsigned>,
}

impl<Signed, Unsigned> Memory<Signed, Unsigned>
where
    Signed: Copy + Clone + Zero,
    Unsigned: AsIndex,
{
    /// Get 32 bits of memory
    pub fn load_word(&self, location: Unsigned) -> Signed {
        self.data
            .get(location.as_index())
            .cloned()
            .unwrap_or_else(Signed::zero)
    }

    /// Set 32 bits of memory
    pub fn store_word(&mut self, location: Unsigned, value: Signed) {
        if location.as_index() > self.data.len() {
            self.data.resize(location.as_index() + 1, Signed::zero());
        }
        self.data[location.as_index()] = value;
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
