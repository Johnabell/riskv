use num::{
    traits::{WrappingAdd, WrappingSub},
    Num,
};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

use crate::instructions::Instruction;
use crate::integer::{AsIndex, AsSigned, AsUnsigned};
use crate::memory::Memory;
use crate::registers::Registers;

// TODO consider making an architecture trait capturing RV32I, RV32E, RV64I, RV128I, etc

trait UnsignedBounds<T: Num>:
    Num + PartialOrd + AsIndex + Shr<u8, Output = Self> + AsSigned<T>
{
}

trait Architecture<Unsigned>:
    Num
    + AsUnsigned<Unsigned>
    + Copy
    + From<i16>
    + From<i32>
    + WrappingAdd
    + WrappingSub
    + PartialOrd
    + From<bool>
    + BitXor<Output = Self>
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Shl<u8, Output = Self>
    + Shr<u8, Output = Self>
where
    Unsigned: UnsignedBounds<Self>,
{
}

impl UnsignedBounds<i32> for u32 {}

impl Architecture<u32> for i32 {}

#[derive(Debug, Default, PartialEq, Eq)]
struct Processor<Signed: Architecture<Unsigned>, Unsigned>
where
    Signed: Architecture<Unsigned>,
    Unsigned: UnsignedBounds<Signed>,
{
    registers: Registers<Signed>,
    // Programme Counter
    pc: Signed,
    memory: Memory<Signed, Unsigned>,
}

impl<Signed, Unsigned> Processor<Signed, Unsigned>
where
    Signed: Architecture<Unsigned>,
    Unsigned: UnsignedBounds<Signed>,
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
            Instruction::ORI { rd, rs1, imm } => {
                self.registers[rd] = self.registers[rs1] | (imm.into())
            }
            Instruction::ANDI { rd, rs1, imm } => {
                self.registers[rd] = self.registers[rs1] & (imm.into())
            }
            Instruction::SLLI { rd, rs1, shamt } => {
                self.registers[rd] = self.registers[rs1] << shamt
            }
            Instruction::SRLI { rd, rs1, shamt } => {
                self.registers[rd] = (self.registers[rs1].as_unsigned() >> shamt).as_signed()
            }
            Instruction::SRAI { rd, rs1, shamt } => {
                self.registers[rd] = self.registers[rs1] >> shamt
            }
            Instruction::ADD { rd, rs1, rs2 } => {
                self.registers[rd] = self.registers[rs1].wrapping_add(&self.registers[rs2])
            }
            Instruction::SUB { rd, rs1, rs2 } => {
                self.registers[rd] = self.registers[rs1].wrapping_sub(&self.registers[rs2])
            }
            Instruction::SLT { rd, rs1, rs2 } => {
                self.registers[rd] = (self.registers[rs1] < self.registers[rs2]).into()
            },
            Instruction::SLTU { rd, rs1, rs2 } => {
                self.registers[rd] =
                    (self.registers[rs1].as_unsigned() < self.registers[rs2].as_unsigned()).into()
            },
            Instruction::LW { rd, rs1, offset } => {
                self.registers[rd] = self.memory.load_word(
                    self.registers[rs1]
                        .wrapping_add(&offset.into())
                        .as_unsigned(),
                )
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
    macro_rules! memory_state {
        ($($location:literal: $value:expr),* $(,)?) => {
            {
                let mut mem = Memory::<i32, u32>::default();
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
    macro_rules! processor_state {
        (
            registers: $register_state:tt
            $(, memory: $memory_state:tt)?
            $(, pc: $program_counter1:expr)?
            $(,)?
        ) => {
            Processor::<i32, u32> {
                registers: register_state!($register_state)
                $(, memory: memory_state!($memory_state))?
                $(, pc: $program_counter1)?
                , ..Default::default()
            }
        };
        (
            {
                registers: $register_state:tt
                $(, memory: $memory_state:tt)?
                $(, pc: $program_counter2:expr)?
                $(,)?
            }
        ) => {
            processor_state!(
                registers: $register_state
                $(, memory: $memory_state)?
                $(, pc: $program_counter2)?
            )
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
    fn execute_seqz() {
        test_execute_many!(
            Instruction::SEQZ(Register::A3, Register::A1),
            changes: {registers: {a1: -1}},
            to: {registers: {a1: -1, a3: 0}},
        );
        test_execute_many!(
            Instruction::SEQZ(Register::A3, Register::A1),
            changes: {registers: {a1: 0}},
            to: {registers: {a1: 0, a3: 1}},
        );
        test_execute_many!(
            Instruction::SEQZ(Register::A3, Register::A1),
            changes: {registers: {a1: 1}},
            to: {registers: {a1: 1, a3: 0}},
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
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 1},
            changes: {registers: {t1: 42}},
            to: {registers: {t1: 42, t4: 0}},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 1},
            changes: {registers: {t1: 0}},
            to: {registers: {t1: 0, t4: 1}},
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
    fn execute_ori() {
        test_execute!(
            Instruction::ORI{rd: Register::S0, rs1: Register::S1, imm: -1},
            changes: {registers: {s1: 42}},
            to: {registers: {s0: -1, s1: 42}},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S2, rs1: Register::S3, imm: 2},
            changes: {registers: {s3: 2}},
            to: {registers: {s2: 2, s3: 2}},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S4, rs1: Register::S5, imm: 8},
            changes: {registers: {s5: 3}},
            to: {registers: {s4: 11, s5: 3}},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S4, rs1: Register::S5, imm: 7},
            changes: {registers: {s5: 19}},
            to: {registers: {s4: 23, s5: 19}},
        );
    }

    #[test]
    fn execute_andi() {
        test_execute!(
            Instruction::ANDI{rd: Register::S0, rs1: Register::S1, imm: -1},
            changes: {registers: {s1: 42}},
            to: {registers: {s0: 42, s1: 42}},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S2, rs1: Register::S3, imm: 2},
            changes: {registers: {s3: 2}},
            to: {registers: {s2: 2, s3: 2}},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S4, rs1: Register::S5, imm: 8},
            changes: {registers: {s5: 3}},
            to: {registers: {s4: 0, s5: 3}},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S4, rs1: Register::S5, imm: 7},
            changes: {registers: {s5: 19}},
            to: {registers: {s4: 3, s5: 19}},
        );
    }

    #[test]
    fn execute_slli() {
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            changes: {registers: {ra: 1}},
            to: {registers: {sp: 32, ra: 1}},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            changes: {registers: {ra: 0}},
            to: {registers: {sp: 0, ra: 0}},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            changes: {registers: {ra: i32::MAX}},
            to: {registers: {sp: -2, ra: i32::MAX}},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            changes: {registers: {ra: 2 << 29}},
            to: {registers: {sp: 0, ra: 2 << 29}},
        );
    }

    #[test]
    fn execute_srli() {
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            changes: {registers: {ra: 64}},
            to: {registers: {sp: 2, ra: 64}},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            changes: {registers: {ra: 0}},
            to: {registers: {sp: 0, ra: 0}},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            changes: {registers: {ra: -1}},
            to: {registers: {sp: i32::MAX, ra: -1}},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            changes: {registers: {ra: -1000}},
            to: {registers: {sp: 2147483148, ra: -1000}},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            changes: {registers: {ra: 42}},
            to: {registers: {sp: 10, ra: 42}},
        );
    }

    #[test]
    fn execute_srai() {
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            changes: {registers: {ra: 64}},
            to: {registers: {sp: 2, ra: 64}},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            changes: {registers: {ra: 0}},
            to: {registers: {sp: 0, ra: 0}},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 10},
            changes: {registers: {ra: -1}},
            to: {registers: {sp: -1, ra: -1}},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 4},
            changes: {registers: {ra: -1000}},
            to: {registers: {sp: -63, ra: -1000}},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            changes: {registers: {ra: 42}},
            to: {registers: {sp: 10, ra: 42}},
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

    #[test]
    fn execute_slt() {
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            changes: {registers: {t1: 42, t3: 43}},
            to: {registers: {t1: 42, t4: 1, t3: 43}},
        );
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            changes: {registers: {t1: 42, t4: 100, t3: 42}},
            to: {registers: {t1: 42, t4: 0, t3: 42}},
        );
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            changes: {registers: {t1: 42, t3: -43}},
            to: {registers: {t1: 42, t4: 0, t3: -43}},
        );
    }

    #[test]
    fn execute_sltu() {
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            changes: {registers: {t1: 42, a4: 43}},
            to: {registers: {t1: 42, t4: 1, a4: 43}},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            changes: {registers: {t1: 42, t4: 100, a4: 42}},
            to: {registers: {t1: 42, t4: 0, a4: 42}},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            changes: {registers: {t1: 42, a4: -43}},
            to: {registers: {t1: 42, t4: 1, a4: -43}},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            changes: {registers: {t1: 42, a4: 1}},
            to: {registers: {t1: 42, t4: 0, a4: 1}},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            changes: {registers: {t1: 0, a4: 1}},
            to: {registers: {t1: 0, t4: 1, a4: 1}},
        );
    }

    #[test]
    fn execute_lw() {
        test_execute!(
            Instruction::LW { rd: Register::T3, rs1: Register::T1, offset: 31, },
            changes: {registers: {t1: 3}, memory: {34: 12}},
            to: {registers: {t1: 3, t3: 12}, memory: {34: 12}},
        );
    }
}
