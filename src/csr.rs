use std::sync::atomic::{AtomicI32, AtomicI64, Ordering::SeqCst};

const CSR_SIZE: usize = 4096;

pub trait CSR {
    type Register;
    fn read_write(&self, index: u16, value: Self::Register) -> Self::Register;
    fn set_bits(&self, index: u16, value: Self::Register) -> Self::Register;
    fn clear_bits(&self, index: u16, value: Self::Register) -> Self::Register;
}

#[derive(Debug)]
pub struct CSR32 {
    registers: Vec<AtomicI32>,
}

#[derive(Debug)]
pub struct CSR64 {
    registers: Vec<AtomicI64>,
}

macro_rules! implement_csr {
    ($struct_name: ty, $register_type:ty) => {
        impl $struct_name {
            fn new() -> Self {
                let mut registers = Vec::default();
                registers.resize_with(CSR_SIZE, Default::default);
                Self { registers }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl CSR for $struct_name {
            type Register = $register_type;

            fn read_write(&self, index: u16, value: Self::Register) -> Self::Register {
                self.registers[index as usize].swap(value, SeqCst)
            }

            fn set_bits(&self, index: u16, value: Self::Register) -> Self::Register {
                self.registers[index as usize].fetch_or(value, SeqCst)
            }

            fn clear_bits(&self, index: u16, value: Self::Register) -> Self::Register {
                self.registers[index as usize].fetch_and(!value, SeqCst)
            }
        }
    };
}

#[cfg(test)]
impl PartialEq for CSR32 {
    fn eq(&self, other: &Self) -> bool {
        for (index, value) in self.registers.iter().enumerate() {
            if value.load(SeqCst) != other.registers[index].load(SeqCst) {
                return false;
            }
        }
        true
    }
}

implement_csr!(CSR32, i32);
implement_csr!(CSR64, i64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_value_zero() {
        let csr_32 = CSR32::default();
        let csr_64 = CSR32::default();

        assert_eq!(csr_32.set_bits(42, 0), 0);
        assert_eq!(csr_64.set_bits(42, 0), 0);
    }

    #[test]
    fn set_bits() {
        let csr_32 = CSR32::default();
        let csr_64 = CSR32::default();

        assert_eq!(csr_32.set_bits(42, -1), 0);
        assert_eq!(csr_64.set_bits(42, -1), 0);
        assert_eq!(csr_32.set_bits(42, 0), -1);
        assert_eq!(csr_64.set_bits(42, 0), -1);
        assert_eq!(csr_32.set_bits(42, -1), -1);
        assert_eq!(csr_64.set_bits(42, -1), -1);
        assert_eq!(csr_32.set_bits(42, 0), -1);
        assert_eq!(csr_64.set_bits(42, 0), -1);
    }

    #[test]
    fn clear_bits() {
        let csr_32 = CSR32::default();
        let csr_64 = CSR32::default();

        assert_eq!(csr_32.set_bits(42, -1), 0);
        assert_eq!(csr_64.set_bits(42, -1), 0);
        assert_eq!(csr_32.clear_bits(42, -1), -1);
        assert_eq!(csr_64.clear_bits(42, -1), -1);
        assert_eq!(csr_32.set_bits(42, 0), 0);
        assert_eq!(csr_64.set_bits(42, 0), 0);
    }
}
