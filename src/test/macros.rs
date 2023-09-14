//! Macros to help write clean and concise tests without lots of boiler plate.
use crate::instructions::Instruction;

/// Used by the test macros to make it possible to treat instructions and
/// pseudoinstructions together as if of the same type for example in a vec or
/// in the `test_execute` macro.
impl IntoIterator for Instruction {
    type Item = Self;

    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

/// A helper macro for creating an instance of [crate::registers::Registers].
///
/// # Example usage
///
/// ```ignore
/// let registers = register_state! {
///   a1: 4,
///   a2: 42,
/// };
/// ```
macro_rules! register_state {
    ($($register:ident: $value:expr),* $(,)?) => {
        crate::registers::Registers {
            $($register: $value,)*
            ..Default::default()
        }
    };
    ({$($register:ident: $value:expr),* $(,)?}) => {
        register_state!($($register: $value,)*)
    };
}

/// A helper macro for creating an instance of [crate::memory::Memory].
///
/// # Example usage
///
/// ```ignore
/// let memory = memory_state! {
///   42: 1,
///   54: 3,
/// };
/// ```
macro_rules! memory_state {
    ($($location:literal: $value:expr),* $(,)?) => {
        {
            let mut mem = crate::memory::Memory::default();
            $(
                mem.store_word($location, $value);
            )*
            mem
        }
    };
    ({$($location:literal: $value:expr),* $(,)?}) => {
        memory_state!($($location: $value,)*)
    };
}

/// A helper macro for creating an instance of [crate::csr::CSR32].
///
/// # Example usage
///
/// ```ignore
/// let csr = csr_state! {
///   42: 1,
///   54: 3,
/// };
/// ```
macro_rules! csr_state {
    ($($index:literal: $value:expr),* $(,)?) => {
        {
            let csr = crate::csr::CSR32::default();
            $(
                csr.read_write($index, $value);
            )*
            csr
        }
    };
    ({$($location:literal: $value:expr),* $(,)?}) => {
        csr_state!($($location: $value,)*)
    };
}

/// A helper macro for creating an instance of [crate::processor::Processor].
///
/// # Example usage
///
/// ```ignore
/// let processor = processor_state! {
///   registers: {t0: 42, s6: 9},
///   memory: {4: 50},
///   csr: {0: 10},
///   pc: 4,
/// };
/// ```
macro_rules! processor_state {
    (
        registers: $register_state:tt
        $(, memory: $memory_state:tt)?
        $(, csr: $csr:tt)?
        $(, pc: $program_counter1:expr)?
        $(,)?
    ) => {
        crate::processor::Processor {
            registers: register_state!($register_state)
            $(, memory: memory_state!($memory_state))?
            $(, csrs: csr_state!($csr))?
            $(, pc: $program_counter1)?
            , ..Default::default()
        }
    };
    (
        {
            registers: $register_state:tt
            $(, memory: $memory_state:tt)?
            $(, csr: $csr:tt)?
            $(, pc: $program_counter2:expr)?
            $(,)?
        }
    ) => {
        processor_state!(
            registers: $register_state
            $(, memory: $memory_state)?
            $(, csr: $csr)?
            $(, pc: $program_counter2)?
        )
    };
}

/// A helper macro for testing the execution of instructions.
///
/// # Example usage
///
/// ```ignore
/// test_execute!(
///     Instruction::CSRRW { rd: Register::A0, rs1: Register::S10, csr: 20 },
///     changes: {registers: {a0: 3, s10: 12}},
///     to: {registers: {a0: 0, s10: 12}, csr: {20: 12}, pc: 4},
/// );
/// ```
macro_rules! test_execute {
    ($instruction:expr, changes: $initial_state:tt, to: $final_state:tt $(,)?) => {{
        // Arrange
        let mut processor = processor_state!($initial_state);

        // Act
        for instruction in $instruction {
            instruction.execute(&mut processor).unwrap();
        }

        // Assert
        let expected_state = processor_state!($final_state);
        assert_eq!(processor, expected_state);
        processor
    }};
    ($instruction:expr, executed_on: $initial_state:tt, throws: $exception:expr $(,)?) => {{
        // Arrange
        let mut processor = processor_state!($initial_state);

        // Act
        let result = $instruction.into_iter().try_for_each(|instruction| {
            instruction.execute(&mut processor)
        });

        assert_eq!(result, Err($exception));
        assert_eq!(processor, processor_state!($initial_state));
    }}
}

/// Macro for creating a vec of instructions from a mixture of instructions and
/// pseudoinstructions.
///
/// # Example usage
///
/// ```ignore
/// let instructions = instructions![
///     Instruction::LI(Register::A0, 21),
///     Instruction::LI(Register::A1, 21),
///     Instruction::ADD {
///         rd: Register::A2,
///         rs1: Register::A1,
///         rs2: Register::A0,
///     },
/// ];
/// ```
macro_rules! instructions {
    ($($instruction:expr),* $(,)*) => {{
        let mut vec = Vec::new();
        $(vec.extend($instruction);)*
        vec
    }};
}

/// A helper macro for testing the execution of a small programme on the
/// [crate::processor::Processor]
///
/// # Example usage
///
/// ```ignore
/// processor_test!(
///     Instruction::LI(Register::A0, 21);
///     Instruction::LI(Register::A1, 21);
///     Instruction::ADD {
///         rd: Register::A2,
///         rs1: Register::A1,
///         rs2: Register::A0,
///     },
///     results_in: {registers: {a0: 21, a1:21, a2: 42}, pc: 12},
/// );
/// ```
macro_rules! processor_test {
    ($($instruction:expr);+ , results_in: $final_state:tt ,) => {
        let instructions = instructions![
            $($instruction,)*
        ];

        let mut processor = Processor::<i32, CSR32>::default();
        processor.store_instructions(0, instructions);

        let initial_memory_state = processor.memory.clone();

        processor.run::<Instruction>();

        let mut expected_final_state = processor_state!($final_state);
        expected_final_state.memory.with_initial_state(initial_memory_state);

        assert_eq!(processor, expected_final_state);
    };
}

pub(crate) use csr_state;
pub(crate) use instructions;
pub(crate) use memory_state;
pub(crate) use processor_state;
pub(crate) use processor_test;
pub(crate) use register_state;
pub(crate) use test_execute;
