//! Core model of the central processing unit.
//!
//! The processor implements execution pipeline.
use crate::csr::ControlStatusRegisters;
use crate::instruction_set::{Exception, InstructionSet};
use crate::integer::AsUsize;
use crate::memory::Memory;
use crate::registers::Registers;

/// The RISC-V machines central processing unit.
///
/// To support different architectures the processor is generic over the
/// register type and CSR type.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Processor<R, CSRs: ControlStatusRegisters<Register = R>> {
    /// The processors registers.
    pub(crate) registers: Registers<R>,
    /// Programme Counter
    pub(crate) pc: R,
    /// The control status registers.
    pub(crate) csrs: CSRs,
    /// The computer memory.
    pub(crate) memory: Memory,
    // TODO add privilege modes
}

impl<R, CSRs: ControlStatusRegisters<Register = R>> Processor<R, CSRs>
where
    R: AsUsize,
{
    /// Execute a single step of the processor pipeline:
    /// `load instruction -> decode instruction -> execute instruction`
    /// returning nothing or an exception if raised.
    #[inline]
    fn inner_step<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(
        &mut self,
    ) -> Result<(), Exception> {
        I::decode(self.memory.load_word(self.pc.as_usize()) as u32)?.execute(self)
    }

    /// Step the process one instruction forward handling any exception which might be raised.
    #[inline]
    pub fn step<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(&mut self) -> ExecutionResult {
        match self.inner_step::<I>() {
            Err(exception) => self.handle_exception(exception),
            Ok(()) => ExecutionResult::Continue,
        }
    }

    /// Run the processor forward until the next [ExecutionResult::Halt].
    pub fn run<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(&mut self) {
        while let ExecutionResult::Continue = self.step::<I>() {}
    }

    /// Run the processor forward from the provided memory location until the
    /// next [ExecutionResult::Halt].
    pub fn run_from<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(
        &mut self,
        initial_mem_location: R,
    ) {
        self.pc = initial_mem_location;
        self.run::<I>()
    }

    /// The processor exception handler.
    ///
    /// At some point this should jump execution to into a specified trap handler.
    #[inline]
    fn handle_exception(&self, _exception: Exception) -> ExecutionResult {
        // TODO handle other types of interrupts
        ExecutionResult::Halt
    }

    /// Store the `instructions` into memory starting from the `initial_memory_location`.
    pub fn store_instructions<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(
        &mut self,
        initial_mem_location: usize,
        instructions: impl IntoIterator<Item = I>,
    ) {
        let final_location = instructions.into_iter().map(I::encode).fold(
            initial_mem_location,
            |location, instruction| {
                let next_location = location + std::mem::size_of_val(&instruction);
                self.memory.store_word(location, instruction as i32);
                next_location
            },
        );
        self.memory.resize::<4>(final_location);
    }
}

/// The result of executing an execution.
pub enum ExecutionResult {
    /// Execution should continue to the next instruction.
    Continue,
    /// Execution should halt and yield control to the caller.
    Halt,
}

#[cfg(test)]
mod test {
    use crate::test::macros::*;
    use crate::{csr::CSR32, instructions::Instruction, registers::Register};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn simple_run() {
        processor_test!(
            Instruction::LI(Register::A0, 21);
            Instruction::LI(Register::A1, 21);
            Instruction::ADD {
                rd: Register::A2,
                rs1: Register::A1,
                rs2: Register::A0,
            },
            results_in: {registers: {a0: 21, a1:21, a2: 42}, pc: 12},
        );
    }
}
