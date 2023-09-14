//! A trait that encapsulates the core behaviour of an instruction set.
//!
//! RISC-V defines a number of base instructions sets (RV32I, RV32E, RV64I, and
//! RV128I) and additional extensions such as M, A, F D, Q, L, C, B, J, T, P, V,
//! or N. This is not a exhaustive list since RISC-V is designed to extensible.
//!
//! To encapsulate this extensibility the [InstructionSet] trait captures the
//! core concept and behaviour of an instruction set.
//!
//! The intention of this crate is to initially provide the implementations of
//! some of the base instructions sets. However to enable alternative
//! implementations and extensions, this trait can be implemented.
use std::fmt::Display;

use crate::{csr::ControlStatusRegisters, processor::Processor};

/// A processor exception
///
/// Different instructions can raise exceptions in the processor for system
/// interrupt.
#[derive(Debug, PartialEq, Eq)]
pub enum Exception {
    /// The processor exception raised when an instruction is not recognised.
    UnimplementedInstruction(u32),

    /// Misaligned Instruction Fetch exception.
    ///
    /// _Note_: Instruction fetch misaligned exceptions are not possible on
    /// machines that support extensions with `16`-bit aligned instructions,
    /// such as the compressed instruction set extension, C.
    MisalignedInstructionFetch,
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnimplementedInstruction(instuction) => f.write_fmt(format_args!(
                "The given instruction is not yet implemented {:#034b}",
                instuction.to_le()
            )),
            Self::MisalignedInstructionFetch => {
                f.write_str("Attempted to fetch an instruction not aligned to a 32-bit boundary")
            }
        }
    }
}

/// The core behaviour of an instruction set.
pub trait InstructionSet: Sized {
    /// The type of the processor's registers.
    ///
    /// Usually, this one of the following
    ///
    /// * i32 (for RV32I or RV32E)
    /// * i64 (for RV64I)
    /// * i128 (for RV128I)
    type RegisterType;

    /// The type of the processors CSRs.
    ///
    /// Usually,
    /// * [crate::csr::CSR32] for a 32-BIT architecture
    /// * [crate::csr::CSR64] for a 64-BIT architecture
    type CSRType: ControlStatusRegisters<Register = Self::RegisterType>;

    /// Decode this 32-bit value as an instruction. TODO: handle larger instructions
    fn decode(raw_instruction: u32) -> Result<Self, Exception>;

    /// Encode the instruction to bytes. TODO: handle larger instructions
    fn encode(self) -> u32;

    /// Run this instruction on the provided processor.
    fn execute(
        self,
        processor: &mut Processor<Self::RegisterType, Self::CSRType>,
    ) -> Result<(), Exception>;

    /// Returns the size of this instruction in number of bytes
    fn instruction_size(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_exception_display() {
        let exception = Exception::UnimplementedInstruction(u32::from_le(
            0b_00001000_00000100_00000010_00000001,
        ));
        assert_eq!(
            exception.to_string(),
            "The given instruction is not yet implemented 0b00001000000001000000001000000001"
        )
    }
}
