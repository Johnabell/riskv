//! Control status registers
//!
//! Control and status register (CSR) is a register that stores various
//! information in CPU. RISC-V defines a separate address space of 4096 CSRs.
//! The RISC-V specification only explicitly allocates a part of address space
//! the rest is OS - implementations specific.
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering::SeqCst};

/// According to the RISC-V specification, the number of control status registers.
const CSR_SIZE: usize = 4096;

/// The control status registers.
pub trait ControlStatusRegisters {
    /// The type of the processor's registers.
    ///
    /// Usually, this one of the following
    ///
    /// * i32 (for RV32I or RV32E)
    /// * i64 (for RV64I)
    /// * i128 (for RV128I)
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
pub struct CSR32 {
    /// A boxed slice of the CSR registers.
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
pub struct CSR64 {
    /// A boxed slice of the CSR registers.
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
        impl core::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!(
                    "{} {{ registers: {{ {} }} }}",
                    stringify!($struct_name),
                    &self
                        .registers
                        .iter()
                        .enumerate()
                        .filter(|(_i, v)| v.load(SeqCst) != 0)
                        .map(|(i, v)| format!("{i}: {}", v.load(SeqCst)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
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
    use pretty_assertions::assert_eq;

    #[test]
    fn debug_format_32() {
        let csr_32 = CSR32::default();
        csr_32.read_write(34, 43);
        csr_32.read_write(85, 58);
        assert_eq!(
            format!("{:?}", csr_32),
            "CSR32 { registers: { 34: 43, 85: 58 } }"
        );
    }

    #[test]
    fn debug_format_64() {
        let csr_64 = CSR64::default();
        csr_64.read_write(14, 41);
        csr_64.read_write(85, 58);
        assert_eq!(
            format!("{:?}", csr_64),
            "CSR64 { registers: { 14: 41, 85: 58 } }"
        );
    }

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
