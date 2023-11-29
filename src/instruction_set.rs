use std::fmt::Display;

use crate::{csr::ControlStatusRegisters, processor::Processor};

#[derive(Debug)]
pub enum Exception {
    UnimplementedInstruction(u32),
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnimplementedInstruction(instuction) => f.write_fmt(format_args!(
                "The given instruction is not yet implemented {:#034b}",
                instuction.to_le()
            )),
        }
    }
}

pub trait InstructionSet: Sized {
    type RegisterType;
    type CSRType: ControlStatusRegisters<Register = Self::RegisterType>;

    /// Decode this 32-bit value as an instruction
    fn decode(raw_instruction: u32) -> Result<Self, Exception>;

    fn execute(
        self,
        processor: &mut Processor<Self::RegisterType, Self::CSRType>,
    ) -> Result<(), Exception>;

    /// Returns the size of this instruction in number of bytes
    fn instruction_size(&self) -> Self::RegisterType;

    const SHIFT_MASK: Self::RegisterType;
}
