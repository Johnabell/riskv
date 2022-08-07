use instructions::Instruction;
use num::Num;

mod instructions;

/// Struct representing the RISK-V Processor.
///
/// The processor contains the following registers.
///
/// | rx | rx' | Register | ABI Name | Description                          | Saved by |
/// | -- | --- | -------- | -------- | ------------------------------------ | -------- |
/// | 0  | -   | x0       | zero     | hardwired zero                       | -        |
/// | 1  | -   | x1       | ra       | return address                       | caller   |
/// | 2  | -   | x2       | sp       | stack pointer                        | callee   |
/// | 3  | -   | x3       | gp       | global pointer                       | -        |
/// | 4  | -   | x4       | tp       | thread pointer                       | -        |
/// | 5  | -   | x5       | t0       | temporary register 0                 | caller   |
/// | 6  | -   | x6       | t1       | temporary register 1                 | caller   |
/// | 7  | -   | x7       | t2       | temporary register 2                 | caller   |
/// | 8  | 0   | x8       | s0 / fp  | saved register 0 / frame pointer     | callee   |
/// | 9  | 1   | x9       | s1       | saved register 1                     | callee   |
/// | 10 | 2   | x10      | a0       | function argument 0 / return value 0 | caller   |
/// | 11 | 3   | x11      | a1       | function argument 1 / return value 1 | caller   |
/// | 12 | 4   | x12      | a2       | function argument 2                  | caller   |
/// | 13 | 5   | x13      | a3       | function argument 3                  | caller   |
/// | 14 | 6   | x14      | a4       | function argument 4                  | caller   |
/// | 15 | 7   | x15      | a5       | function argument 5                  | caller   |
/// | 16 | -   | x16      | a6       | function argument 6                  | caller   |
/// | 17 | -   | x17      | a7       | function argument 7                  | caller   |
/// | 18 | -   | x18      | s2       | saved register 2                     | callee   |
/// | 19 | -   | x19      | s3       | saved register 3                     | callee   |
/// | 20 | -   | x20      | s4       | saved register 4                     | callee   |
/// | 21 | -   | x21      | s5       | saved register 5                     | callee   |
/// | 22 | -   | x22      | s6       | saved register 6                     | callee   |
/// | 23 | -   | x23      | s7       | saved register 7                     | callee   |
/// | 24 | -   | x24      | s8       | saved register 8                     | callee   |
/// | 25 | -   | x25      | s9       | saved register 9                     | callee   |
/// | 26 | -   | x26      | s10      | saved register 10                    | callee   |
/// | 27 | -   | x27      | s11      | saved register 11                    | callee   |
/// | 28 | -   | x28      | t3       | temporary register 3                 | caller   |
/// | 29 | -   | x29      | t4       | temporary register 4                 | caller   |
/// | 30 | -   | x30      | t5       | temporary register 5                 | caller   |
/// | 31 | -   | x31      | t6       | temporary register 6                 | caller   |
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Registers<T>
where
    T: Num,
{
    ra: T,
    sp: T,
    gp: T,
    tp: T,
    t0: T,
    t1: T,
    t2: T,
    s0: T,
    s1: T,
    a0: T,
    a1: T,
    a2: T,
    a3: T,
    a4: T,
    a5: T,
    a6: T,
    a7: T,
    s2: T,
    s3: T,
    s4: T,
    s5: T,
    s6: T,
    s7: T,
    s8: T,
    s9: T,
    s10: T,
    s11: T,
    t3: T,
    t4: T,
    t5: T,
    t6: T,
}

// TODO consider making an architecture trait capturing RV32I, RV32E, RV64I, RV128I, etc
pub trait Architecture: Num {}

pub struct Processor<T: Architecture> {
    registers: Registers<T>,
    // Programme Counter
    pc: T,
    instructions: Vec<Instruction>,
}

impl<T> Registers<T>
where
    T: Num + Copy + From<u16> + From<u32>,
{
    /// Executes a single instruction on the processor
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD { rd, rs1, rs2 } => self[rd] = self[rs1] + self[rs2],
            Instruction::ADDI { rd, rs1, imm } => self[rd] = self[rs1] + imm.into(),
            Instruction::SUB { rd, rs1, rs2 } => self[rd] = self[rs1] - self[rs2],
            Instruction::LI { rd, imm } => self[rd] = imm.into(),
        }
    }
}

impl<T> std::ops::Index<u8> for Registers<T>
where
    T: Num,
{
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => todo!(),
            1 => &self.ra,
            2 => &self.sp,
            3 => &self.gp,
            4 => &self.tp,
            5 => &self.t0,
            6 => &self.t1,
            7 => &self.t2,
            8 => &self.s0,
            9 => &self.s1,
            10 => &self.a0,
            11 => &self.a1,
            12 => &self.a2,
            13 => &self.a3,
            14 => &self.a4,
            15 => &self.a5,
            16 => &self.a6,
            17 => &self.a7,
            18 => &self.s2,
            19 => &self.s3,
            20 => &self.s4,
            21 => &self.s5,
            22 => &self.s6,
            23 => &self.s7,
            24 => &self.s8,
            25 => &self.s9,
            26 => &self.s10,
            27 => &self.s11,
            28 => &self.t3,
            29 => &self.t4,
            30 => &self.t5,
            31 => &self.t6,
            _ => panic!("Out of bounds"),
        }
    }
}

impl<T> std::ops::Index<Register> for Registers<T>
where
    T: Num,
{
    type Output = T;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::ZERO => todo!(),
            Register::RA => &self.ra,
            Register::SP => &self.sp,
            Register::GP => &self.gp,
            Register::TP => &self.tp,
            Register::T0 => &self.t0,
            Register::T1 => &self.t1,
            Register::T2 => &self.t2,
            Register::S0 => &self.s0,
            Register::S1 => &self.s1,
            Register::A0 => &self.a0,
            Register::A1 => &self.a1,
            Register::A2 => &self.a2,
            Register::A3 => &self.a3,
            Register::A4 => &self.a4,
            Register::A5 => &self.a5,
            Register::A6 => &self.a6,
            Register::A7 => &self.a7,
            Register::S2 => &self.s2,
            Register::S3 => &self.s3,
            Register::S4 => &self.s4,
            Register::S5 => &self.s5,
            Register::S6 => &self.s6,
            Register::S7 => &self.s7,
            Register::S8 => &self.s8,
            Register::S9 => &self.s9,
            Register::S10 => &self.s10,
            Register::S11 => &self.s11,
            Register::T3 => &self.t3,
            Register::T4 => &self.t4,
            Register::T5 => &self.t5,
            Register::T6 => &self.t6,
        }
    }
}

impl<T> std::ops::IndexMut<u8> for Registers<T>
where
    T: Num,
{
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => todo!(),
            1 => &mut self.ra,
            2 => &mut self.sp,
            3 => &mut self.gp,
            4 => &mut self.tp,
            5 => &mut self.t0,
            6 => &mut self.t1,
            7 => &mut self.t2,
            8 => &mut self.s0,
            9 => &mut self.s1,
            10 => &mut self.a0,
            11 => &mut self.a1,
            12 => &mut self.a2,
            13 => &mut self.a3,
            14 => &mut self.a4,
            15 => &mut self.a5,
            16 => &mut self.a6,
            17 => &mut self.a7,
            18 => &mut self.s2,
            19 => &mut self.s3,
            20 => &mut self.s4,
            21 => &mut self.s5,
            22 => &mut self.s6,
            23 => &mut self.s7,
            24 => &mut self.s8,
            25 => &mut self.s9,
            26 => &mut self.s10,
            27 => &mut self.s11,
            28 => &mut self.t3,
            29 => &mut self.t4,
            30 => &mut self.t5,
            31 => &mut self.t6,
            _ => panic!("Out of bounds"),
        }
    }
}

impl<T> std::ops::IndexMut<Register> for Registers<T>
where
    T: Num,
{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::ZERO => todo!(),
            Register::RA => &mut self.ra,
            Register::SP => &mut self.sp,
            Register::GP => &mut self.gp,
            Register::TP => &mut self.tp,
            Register::T0 => &mut self.t0,
            Register::T1 => &mut self.t1,
            Register::T2 => &mut self.t2,
            Register::S0 => &mut self.s0,
            Register::S1 => &mut self.s1,
            Register::A0 => &mut self.a0,
            Register::A1 => &mut self.a1,
            Register::A2 => &mut self.a2,
            Register::A3 => &mut self.a3,
            Register::A4 => &mut self.a4,
            Register::A5 => &mut self.a5,
            Register::A6 => &mut self.a6,
            Register::A7 => &mut self.a7,
            Register::S2 => &mut self.s2,
            Register::S3 => &mut self.s3,
            Register::S4 => &mut self.s4,
            Register::S5 => &mut self.s5,
            Register::S6 => &mut self.s6,
            Register::S7 => &mut self.s7,
            Register::S8 => &mut self.s8,
            Register::S9 => &mut self.s9,
            Register::S10 => &mut self.s10,
            Register::S11 => &mut self.s11,
            Register::T3 => &mut self.t3,
            Register::T4 => &mut self.t4,
            Register::T5 => &mut self.t5,
            Register::T6 => &mut self.t6,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    /// hardwired zero
    ZERO = 0,
    /// return address
    RA = 1,
    /// stack pointer
    SP = 2,
    /// global pointer
    GP = 3,
    /// thread pointer
    TP = 4,
    /// temporary register 0
    T0 = 5,
    /// temporary register 1
    T1 = 6,
    /// temporary register 2
    T2 = 7,
    /// saved register 0 / frame pointer
    S0 = 8,
    /// saved register 1
    S1 = 9,
    /// function argument 0 / return value 0
    A0 = 10,
    /// function argument 1 / return value 1
    A1 = 11,
    /// function argument 2
    A2 = 12,
    /// function argument 3
    A3 = 13,
    /// function argument 4
    A4 = 14,
    /// function argument 5
    A5 = 15,
    /// function argument 6
    A6 = 16,
    /// function argument 7
    A7 = 17,
    /// saved register 2
    S2 = 18,
    /// saved register 3
    S3 = 19,
    /// saved register 4
    S4 = 20,
    /// saved register 5
    S5 = 21,
    /// saved register 6
    S6 = 22,
    /// saved register 7
    S7 = 23,
    /// saved register 8
    S8 = 24,
    /// saved register 9
    S9 = 25,
    /// saved register 10
    S10 = 26,
    /// saved register 11
    S11 = 27,
    /// temporary register 3
    T3 = 28,
    /// temporary register 4
    T4 = 29,
    /// temporary register 5
    T5 = 30,
    /// temporary register 6
    T6 = 31,
}

impl From<u8> for Register {
    fn from(value: u8) -> Register {
        match value & 0b_0001_1111 {
            0 => Register::ZERO,
            1 => Register::RA,
            2 => Register::SP,
            3 => Register::GP,
            4 => Register::TP,
            5 => Register::T0,
            6 => Register::T1,
            7 => Register::T2,
            8 => Register::S0,
            9 => Register::S1,
            10 => Register::A0,
            11 => Register::A1,
            12 => Register::A2,
            13 => Register::A3,
            14 => Register::A4,
            15 => Register::A5,
            16 => Register::A6,
            17 => Register::A7,
            18 => Register::S2,
            19 => Register::S3,
            20 => Register::S4,
            21 => Register::S5,
            22 => Register::S6,
            23 => Register::S7,
            24 => Register::S8,
            25 => Register::S9,
            26 => Register::S10,
            27 => Register::S11,
            28 => Register::T3,
            29 => Register::T4,
            30 => Register::T5,
            31 => Register::T6,
            _ => unreachable!("Already masked the value"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! processor_state {
        ($($register:ident: $value:expr),* $(,)?) => {
            Registers::<u32> {
                $($register: $value,)*
                ..Default::default()
            }
        };
        ({$($register:ident: $value:expr),* $(,)?}) => {
            processor_state!($($register: $value,)*)
        };
    }
    macro_rules! test_execute {
        ($instruction:expr, changes: $initial_state:tt, to: $final_state:tt $(,)?) => {
            // Arrange
            let mut processor = processor_state!($initial_state);

            // Act
            processor.execute($instruction);

            // Assert
            assert_eq!(processor, processor_state!($final_state));
        };
    }

    #[test]
    fn execute_load_immediate() {
        test_execute!(
            Instruction::LI{rd: Register::T1, imm: 42},
            changes: {},
            to: {t1: 42},
        );
    }

    #[test]
    fn execute_add() {
        test_execute!(
            Instruction::ADD{rd: Register::T3, rs1: Register::T1, rs2: Register::T2},
            changes: {t1: 21, t2: 21},
            to: {t1: 21, t2: 21, t3: 42},
        );
    }

    #[test]
    fn execute_addi() {
        test_execute!(
            Instruction::ADDI{rd: Register::T4, rs1: Register::T1, imm: 42},
            changes: {t1: 42},
            to: {t1: 42, t4: 84},
        );
    }

    #[test]
    fn execute_sub() {
        test_execute!(
            Instruction::SUB { rd: Register::T3, rs1: Register::T1, rs2: Register::T2, },
            changes: {t1: 45, t2: 3},
            to: {t1: 45, t2: 3, t3: 42},
        );
    }

    #[test]
    fn from_u8() {
        assert_eq!(Register::from(31), Register::T6);
        assert_eq!(Register::from(32), Register::ZERO);
    }
}
