use std::sync::atomic::{AtomicI32, AtomicI64, Ordering::SeqCst};

/// According to the RISC-V specification, the number of control status registers.
const CSR_SIZE: usize = 4096;

/// The control status registers.
pub trait CSR {
    type Register;
    /// Swaps the value for the value at `index`.
    ///
    /// Panics if index is out of bounds (>`CSR_SIZE`)
    fn read_write(&self, index: u16, value: Self::Register) -> Self::Register;
    /// Set the bits in the `value` bit mask.
    ///
    /// Panics if index is out of bounds (>`CSR_SIZE`)
    fn set_bits(&self, index: u16, value: Self::Register) -> Self::Register;
    /// Clears the bits in the `value` bit mask.
    ///
    /// Panics if index is out of bounds (>`CSR_SIZE`)
    fn clear_bits(&self, index: u16, value: Self::Register) -> Self::Register;
}

/// The 32-bit control status registers.
///
/// This is currently a naïve implementation which simply is a heap allocated
/// fixed size array.
///
/// For example, the following instructions represent reading certain system
/// attributes from the CSRs
/// - `RDCYCLE[H]` - a pseudo-instruction to read the cycle CSR which holds the
///   count of the number of clock cycles executed by the processor core on
///   which the hart is running form an arbitrary time in the past
/// - `RDTIME[H]` - a pseudo-instruction to read the time CSR which counts the
///   wall-clock real time that has passed from an arbitrary start time in the
///   past.
/// - `RDINSTRET[H]` - a pseudo-instruction to read the instret CSR which counts
///   the number of instructions retired by this hart from some arbitrary start
///   point in the past.
///
/// Additionally some registers are read only.
#[derive(Debug)]
pub struct CSR32 {
    registers: Vec<AtomicI32>,
}

/// The 64-bit control status registers.
///
/// This is currently a naïve implementation which simply is a heap allocated
/// fixed size array.
///
/// For example, the following instructions represent reading certain system
/// attributes from the CSRs
/// - `RDCYCLE` - a pseudo-instruction to read the cycle CSR which holds the
///   count of the number of clock cycles executed by the processor core on
///   which the hart is running form an arbitrary time in the past
/// - `RDTIME` - a pseudo-instruction to read the time CSR which counts the
///   wall-clock real time that has passed from an arbitrary start time in the
///   past.
/// - `RDINSTRET` - a pseudo-instruction to read the instret CSR which counts
///   the number of instructions retired by this hart from some arbitrary start
///   point in the past.
///
/// Additionally some registers are read only.
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
