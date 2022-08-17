use std::{marker::PhantomData, ops::BitXor};

use super::instructions::Instruction;
use super::integer::AsUnsigned;
use super::registers::Registers;
use num::{
    traits::{WrappingAdd, WrappingSub},
    Num,
};

// TODO consider making an architecture trait capturing RV32I, RV32E, RV64I, RV128I, etc
trait Architecture<Unsigned: Num + PartialOrd>: Num + AsUnsigned<Unsigned> {}
impl Architecture<u32> for i32 {}

#[derive(Debug, Default, PartialEq, Eq)]
struct Processor<Unsigned: Num + PartialOrd, Signed: Architecture<Unsigned>> {
    registers: Registers<Signed>,
    // Programme Counter
    pc: Signed,
    memory: Vec<Signed>,
    _marker: PhantomData<Unsigned>,
}

impl<Unsigned, Signed> Processor<Unsigned, Signed>
where
    Signed: Architecture<Unsigned>
        + Copy
        + From<i16>
        + From<i32>
        + WrappingAdd
        + WrappingSub
        + PartialOrd
        + From<bool>
        + BitXor<Output = Signed>,
    Unsigned: Num + PartialOrd,
{
    /// Executes a single instruction on the processor
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LUI { rd, imm } => self.registers[rd] = (imm << 12).into(),
            Instruction::AUIPC { rd, imm } => self.registers[rd] = self.pc + (imm << 12).into(),
            Instruction::ADDI { rd, rs1, imm } => {
                self.registers[rd] = self.registers[rs1].wrapping_add(&imm.into())
            }
            Instruction::SLTI { rd, rs1, imm } => {
                self.registers[rd] = (self.registers[rs1] < imm.into()).into()
            }
            Instruction::SLTIU { rd, rs1, imm } => {
                self.registers[rd] =
                    (self.registers[rs1].as_unsigned() < Signed::from(imm).as_unsigned()).into()
            }
            Instruction::XORI { rd, rs1, imm } => {
                self.registers[rd] = self.registers[rs1] ^ (imm.into())
            }
            Instruction::ADD { rd, rs1, rs2 } => {
                self.registers[rd] = self.registers[rs1].wrapping_add(&self.registers[rs2])
            }
            Instruction::SUB { rd, rs1, rs2 } => {
                self.registers[rd] = self.registers[rs1].wrapping_sub(&self.registers[rs2])
            }
        }
    }

    fn execute_many(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for instruction in instructions {
            self.execute(instruction);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::i12;
    use crate::registers::{Register, Registers};

    macro_rules! register_state {
        ($($register:ident: $value:expr),* $(,)?) => {
            Registers::<i32> {
                $($register: $value,)*
                ..Default::default()
            }
        };
        ({$($register:ident: $value:expr),* $(,)?}) => {
            register_state!($($register: $value,)*)
        };
    }
    macro_rules! processor_state {
        (registers: $register_state:tt,  $($detail:ident: $value:expr),* $(,)?) => {
            Processor::<u32, i32> {
                registers: register_state!($register_state),
                $($detail: $value,)*
                ..Default::default()
            }
        };
        ({registers: $register_state:tt $(,)? $($detail:ident: $value:expr),* $(,)?}) => {
            processor_state!(registers: $register_state, $($detail: $value,)*)
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
    macro_rules! test_execute_many {
        ($instructions:expr, changes: $initial_state:tt, to: $final_state:tt $(,)?) => {
            // Arrange
            let mut processor = processor_state!($initial_state);

            // Act
            processor.execute_many($instructions);

            // Assert
            assert_eq!(processor, processor_state!($final_state));
        };
    }

    #[test]
    fn execute_li() {
        for i in [
            0,
            i12::MIN as i32,
            i12::MAX as i32,
            i12::MIN as i32 - 1,
            i12::MAX as i32 + 1,
            i32::MAX,
            i32::MIN,
        ] {
            test_execute_many!(
                Instruction::LI(Register::T1, i),
                changes: {registers: {}},
                to: {registers: {t1: i}},
            );
        }
    }

    #[test]
    fn execute_not() {
        test_execute_many!(
            Instruction::NOT(Register::A5, Register::A6),
            changes: {registers: {a6: -1}},
            to: {registers: {a5: 0, a6: -1}},
        );
        test_execute_many!(
            Instruction::NOT(Register::A5, Register::A6),
            changes: {registers: {a6: 0}},
            to: {registers: {a5: -1, a6: 0}},
        );
        test_execute_many!(
            Instruction::NOT(Register::A5, Register::A6),
            changes: {registers: {a6: 42}},
            to: {registers: {a5: -43, a6: 42}},
        );
    }

    #[test]
    fn execute_neg() {
        test_execute_many!(
            Instruction::NEG(Register::S5, Register::S6),
            changes: {registers: {s6: -1}},
            to: {registers: {s5: 1, s6: -1}},
        );
        test_execute_many!(
            Instruction::NEG(Register::S5, Register::S6),
            changes: {registers: {s6: -1}},
            to: {registers: {s5: 1, s6: -1}},
        );
        test_execute_many!(
            Instruction::NEG(Register::S5, Register::S6),
            changes: {registers: {s6: 42}},
            to: {registers: {s5: -42, s6: 42}},
        );
        test_execute_many!(
            Instruction::NEG(Register::S5, Register::S6),
            changes: {registers: {s6: 0}},
            to: {registers: {s5: 0, s6: 0}},
        );
    }

    #[test]
    fn execute_mov() {
        test_execute_many!(
            Instruction::MOV(Register::T5, Register::T6),
            changes: {registers: {t6: -1}},
            to: {registers: {t5: -1, t6: -1}},
        );
    }

    #[test]
    fn execute_nop() {
        test_execute_many!(
            Instruction::NOP,
            changes: {registers: {}},
            to: {registers: {}},
        );
    }

    #[test]
    fn execute_lui() {
        test_execute!(
            Instruction::LUI { rd: Register::S11, imm: 0x2BAAA },
            changes: {registers: {}},
            to: {registers: {s11: 0x2BAA_A000}},
        );
        test_execute!(
            Instruction::LUI { rd: Register::S11, imm: 0xDEAD_B },
            changes: {registers: {}},
            to: {registers: {s11: i32::from_be_bytes([0xDE, 0xAD, 0xB0, 0x00])}},
        );
    }

    #[test]
    fn execute_auipc() {
        test_execute!(
            Instruction::AUIPC { rd: Register::SP, imm: 0x2BAAA },
            changes: {registers: {}, pc: 0x0000_0AAD},
            to: {registers: {sp: 0x2BAA_AAAD}, pc: 0x0000_0AAD},
        );
        test_execute!(
            Instruction::AUIPC { rd: Register::SP, imm: 0xDEAD_B },
            changes: {registers: {}, pc: 0x0000_0EAF},
            to: {registers: {sp: i32::from_be_bytes([0xDE, 0xAD, 0xBE, 0xAF])}, pc: 0x0000_0EAF},
        );
    }

    #[test]
    fn execute_addi() {
        test_execute!(
            Instruction::ADDI{rd: Register::T4, rs1: Register::T1, imm: 42},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 84}},
        );
    }

    #[test]
    fn execute_slti() {
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: 43},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 1}},
        );
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: 42},
            changes: {registers: {t1: 42, t4: 100}},
            to: {registers: {t1: 42, t4: 0}},
        );
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: -43},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 0}},
        );
    }

    #[test]
    fn execute_sltiu() {
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 43},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 1}},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 42},
            changes: {registers: {t1: 42, t4: 100}},
            to: {registers: {t1: 42, t4: 0}},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: -43},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 1}},
        );
    }

    #[test]
    fn execute_xori() {
        test_execute!(
            Instruction::XORI{rd: Register::A0, rs1: Register::A1, imm: -1},
            changes: {registers: {a1: 42}},
            to: {registers: {a0: !(42), a1: 42}},
        );
        test_execute!(
            Instruction::XORI{rd: Register::A2, rs1: Register::A3, imm: 1},
            changes: {registers: {a3: 2}},
            to: {registers: {a2: 3, a3: 2}},
        );
    }

    #[test]
    fn execute_add() {
        test_execute!(
            Instruction::ADD{rd: Register::T3, rs1: Register::T1, rs2: Register::T2},
            changes: {registers: {t1: 21, t2: 21}},
            to: {registers: {t1: 21, t2: 21, t3: 42}},
        );
    }

    #[test]
    fn execute_sub() {
        test_execute!(
            Instruction::SUB { rd: Register::T3, rs1: Register::T1, rs2: Register::T2, },
            changes: {registers: {t1: 45, t2: 3}},
            to: {registers: {t1: 45, t2: 3, t3: 42}},
        );
    }
}
