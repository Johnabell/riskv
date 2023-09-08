use crate::csr::ControlStatusRegisters;
use crate::instruction_set::{Exception, InstructionSet};
use crate::integer::AsUsize;
use crate::memory::Memory;
use crate::registers::Registers;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Processor<R, CSRs: ControlStatusRegisters<Register = R>> {
    pub(crate) registers: Registers<R>,
    // Programme Counter
    pub(crate) pc: R,
    pub(crate) csrs: CSRs,
    pub(crate) memory: Memory,
}

impl<R, CSRs: ControlStatusRegisters<Register = R>> Processor<R, CSRs>
where
    R: AsUsize,
{
    #[inline]
    fn inner_step<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(
        &mut self,
    ) -> Result<(), Exception> {
        I::decode(self.memory.load_word(self.pc.as_usize()) as u32)?.execute(self)
    }

    #[inline]
    pub fn step<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(&mut self) -> ExecutionResult {
        match self.inner_step::<I>() {
            Err(exception) => self.handle_exception(exception),
            Ok(()) => ExecutionResult::Continue,
        }
    }

    pub fn run<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(&mut self) {
        while let ExecutionResult::Continue = self.step::<I>() {}
    }

    pub fn run_from<I: InstructionSet<RegisterType = R, CSRType = CSRs>>(
        &mut self,
        initial_mem_location: R,
    ) {
        self.pc = initial_mem_location;
        self.run::<I>()
    }

    #[inline]
    fn handle_exception(&self, _exception: Exception) -> ExecutionResult {
        // TODO handle other types of interrupts
        ExecutionResult::Finished
    }

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

pub enum ExecutionResult {
    Continue,
    Finished,
}

#[cfg(test)]
mod test {
    use crate::{csr::CSR32, instructions::Instruction, registers::Register};
    use pretty_assertions::assert_eq;

    use super::*;

    /// Macro for creating a programme from a mixture of instructions and
    /// pseudoinstructions.
    macro_rules! instructions {
        ($($instruction:expr),* $(,)*) => {{
            let mut vec = Vec::new();
            $(vec.extend($instruction);)*
            vec
        }};
    }

    /// Used by the macro above to make it possible to include instruction and
    /// pseudoinstructions together as if of the same time in the vec type syntax
    impl IntoIterator for Instruction {
        type Item = Self;

        type IntoIter = std::iter::Once<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            std::iter::once(self)
        }
    }

    #[test]
    // TODO: create macro for making mini programme test like this (reuse the
    // existing test macros and move somewhere common.
    fn simple_run() {
        // Arrange
        let instructions = instructions![
            Instruction::LI(Register::A0, 21),
            Instruction::LI(Register::A1, 21),
            Instruction::ADD {
                rd: Register::A2,
                rs1: Register::A1,
                rs2: Register::A0,
            },
        ];

        let mut processor = Processor::<i32, CSR32>::default();
        processor.store_instructions(0, instructions);

        let initial_memory_state = processor.memory.clone();

        // Act
        processor.run::<Instruction>();

        // Assert
        let expected_final_state = Processor::<i32, CSR32> {
            memory: initial_memory_state,
            registers: Registers::<i32> {
                a0: 21,
                a1: 21,
                a2: 42,
                ..Default::default()
            },
            pc: 12,
            ..Default::default()
        };

        assert_eq!(processor, expected_final_state)
    }
}
