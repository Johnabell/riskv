use crate::{csr::ControlStatusRegisters, processor::Processor};

#[derive(Debug)]
pub enum Exception {}

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
