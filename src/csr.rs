use std::sync::atomic::{AtomicI32, AtomicI64, Ordering::SeqCst};

/// According to the RISC-V specification, the number of control status registers.
const CSR_SIZE: usize = 4096;

/// The control status registers.
pub trait ControlStatusRegisters {
    type Register;
    /// Reads the value of the CSR.
    ///
    /// Panics if index is out of bounds (>`CSR_SIZE`)
    fn read(&self, index: u16) -> Self::Register;
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
    registers: Box<[AtomicI32]>,
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
    registers: Box<[AtomicI64]>,
}

/// This macro implement `new`, `Default::default()`, and
/// `ControlStatusRegisters` for the given struct.
///
/// The struct must contain a single field called `registers` containing a
/// boxed slice of some `Atomic` integer.
macro_rules! implement_csr {
    ($struct_name: ty, $register_type:ty) => {
        impl $struct_name {
            fn new() -> Self {
                let mut registers = Vec::default();
                registers.resize_with(CSR_SIZE, Default::default);
                Self {
                    registers: registers.into_boxed_slice(),
                }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl ControlStatusRegisters for $struct_name {
            type Register = $register_type;

            fn read(&self, index: u16) -> Self::Register {
                self.registers[index as usize].load(SeqCst)
            }

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
        self.registers.len() == other.registers.len()
            && self
                .registers
                .iter()
                .zip(other.registers.iter())
                .all(|(a, b)| a.load(SeqCst) == b.load(SeqCst))
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
        let csr_64 = CSR64::default();

        assert_eq!(csr_32.read(42), 0);
        assert_eq!(csr_64.read(42), 0);
    }

    #[test]
    fn set_bits_32() {
        let csr_32 = CSR32::default();

        assert_eq!(csr_32.set_bits(42, -1), 0);
        assert_eq!(csr_32.read(42), -1);
        assert_eq!(csr_32.set_bits(42, -1), -1);
        assert_eq!(csr_32.read(42), -1);
    }

    #[test]
    fn set_bits_64() {
        let csr_64 = CSR64::default();

        assert_eq!(csr_64.set_bits(42, -1), 0);
        assert_eq!(csr_64.read(42), -1);
        assert_eq!(csr_64.set_bits(42, -1), -1);
        assert_eq!(csr_64.read(42), -1);
    }

    #[test]
    fn clear_bits_32() {
        let csr_32 = CSR32::default();

        assert_eq!(csr_32.set_bits(42, -1), 0);
        assert_eq!(csr_32.clear_bits(42, -1), -1);
        assert_eq!(csr_32.read(42), 0);
    }

    #[test]
    fn clear_bits_64() {
        let csr_64 = CSR64::default();

        assert_eq!(csr_64.set_bits(42, -1), 0);
        assert_eq!(csr_64.clear_bits(42, -1), -1);
        assert_eq!(csr_64.read(42), 0);
    }

    #[test]
    fn read_write_32() {
        let csr_32 = CSR32::default();

        assert_eq!(csr_32.read_write(42, 100), 0);
        assert_eq!(csr_32.read_write(42, 50), 100);
    }

    #[test]
    fn read_write_64() {
        let csr_64 = CSR64::default();

        assert_eq!(csr_64.read_write(42, 100), 0);
        assert_eq!(csr_64.read_write(42, 50), 100);
    }
}
