use crate::csr::{ControlStatusRegisters, CSR32};
use crate::instruction_set::{Exception, InstructionSet};
use crate::integer::{AsSigned, AsUnsigned};
use crate::processor::Processor;
use crate::registers::Register;

use super::Instruction;

impl InstructionSet for Instruction {
    const SHIFT_MASK: i32 = 0b_00000000_00000000_00000000_00011111;

    type RegisterType = i32;
    type CSRType = CSR32;

    fn decode(raw_instruction: u32) -> Result<Self, Exception> {
        Ok(raw_instruction.into())
    }

    fn execute(
        self,
        processor: &mut Processor<Self::RegisterType, Self::CSRType>,
    ) -> Result<(), Exception> {
        match self {
            Instruction::LUI { rd, imm } => processor.registers[rd] = imm << 12,
            Instruction::AUIPC { rd, imm } => processor.registers[rd] = processor.pc + (imm << 12),
            Instruction::ADDI { rd, rs1, imm } => {
                processor.registers[rd] = processor.registers[rs1].wrapping_add(imm.into())
            }
            Instruction::SLTI { rd, rs1, imm } => {
                processor.registers[rd] = (processor.registers[rs1] < imm.into()).into()
            }
            Instruction::SLTIU { rd, rs1, imm } => {
                processor.registers[rd] = (processor.registers[rs1].as_unsigned()
                    < Self::RegisterType::from(imm).as_unsigned())
                .into()
            }
            Instruction::XORI { rd, rs1, imm } => {
                processor.registers[rd] = processor.registers[rs1] ^ Self::RegisterType::from(imm)
            }
            Instruction::ORI { rd, rs1, imm } => {
                processor.registers[rd] = processor.registers[rs1] | Self::RegisterType::from(imm)
            }
            Instruction::ANDI { rd, rs1, imm } => {
                processor.registers[rd] = processor.registers[rs1] & Self::RegisterType::from(imm)
            }
            Instruction::SLLI { rd, rs1, shamt } => {
                processor.registers[rd] = processor.registers[rs1] << shamt
            }
            Instruction::SRLI { rd, rs1, shamt } => {
                processor.registers[rd] =
                    (processor.registers[rs1].as_unsigned() >> shamt).as_signed()
            }
            Instruction::SRAI { rd, rs1, shamt } => {
                processor.registers[rd] = processor.registers[rs1] >> shamt
            }
            Instruction::ADD { rd, rs1, rs2 } => {
                processor.registers[rd] =
                    processor.registers[rs1].wrapping_add(processor.registers[rs2])
            }
            Instruction::SUB { rd, rs1, rs2 } => {
                processor.registers[rd] =
                    processor.registers[rs1].wrapping_sub(processor.registers[rs2])
            }
            Instruction::SLL { rd, rs1, rs2 } => {
                processor.registers[rd] =
                    processor.registers[rs1] << (processor.registers[rs2] & Self::SHIFT_MASK)
            }
            Instruction::SLT { rd, rs1, rs2 } => {
                processor.registers[rd] =
                    (processor.registers[rs1] < processor.registers[rs2]).into()
            }
            Instruction::SLTU { rd, rs1, rs2 } => {
                processor.registers[rd] = (processor.registers[rs1].as_unsigned()
                    < processor.registers[rs2].as_unsigned())
                .into()
            }
            Instruction::XOR { rd, rs1, rs2 } => {
                processor.registers[rd] = processor.registers[rs1] ^ processor.registers[rs2]
            }
            Instruction::SRL { rd, rs1, rs2 } => {
                processor.registers[rd] = (processor.registers[rs1].as_unsigned()
                    >> (processor.registers[rs2] & Self::SHIFT_MASK))
                    .as_signed()
            }
            Instruction::SRA { rd, rs1, rs2 } => {
                processor.registers[rd] =
                    processor.registers[rs1] >> (processor.registers[rs2] & Self::SHIFT_MASK)
            }
            Instruction::OR { rd, rs1, rs2 } => {
                processor.registers[rd] = processor.registers[rs1] | processor.registers[rs2]
            }
            Instruction::AND { rd, rs1, rs2 } => {
                processor.registers[rd] = processor.registers[rs1] & processor.registers[rs2]
            }
            Instruction::CSRRW {
                rd,
                rs1: Register::ZERO,
                csr,
            } => processor.registers[rd] = processor.csrs.read(csr),
            Instruction::CSRRW { rd, rs1, csr } => {
                processor.registers[rd] = processor.csrs.read_write(csr, processor.registers[rs1])
            }
            Instruction::CSRRS { rd, rs1, csr } => {
                processor.registers[rd] = processor.csrs.set_bits(csr, processor.registers[rs1])
            }
            Instruction::CSRRC { rd, rs1, csr } => {
                processor.registers[rd] = processor.csrs.clear_bits(csr, processor.registers[rs1])
            }
            Instruction::CSRRWI { rd, csr, imm } => {
                processor.registers[rd] = processor
                    .csrs
                    .read_write(csr, Self::RegisterType::from(imm))
            }
            Instruction::CSRRSI { rd, csr, imm } => {
                processor.registers[rd] =
                    processor.csrs.set_bits(csr, Self::RegisterType::from(imm))
            }
            Instruction::CSRRCI { rd, csr, imm } => {
                processor.registers[rd] = processor
                    .csrs
                    .clear_bits(csr, Self::RegisterType::from(imm))
            }
            Instruction::LB { rd, rs1, offset } => {
                processor.registers[rd] = processor.memory.load_byte(
                    processor.registers[rs1]
                        .wrapping_add(offset.into())
                        .as_unsigned() as usize,
                ) as Self::RegisterType
            }
            Instruction::LH { rd, rs1, offset } => {
                processor.registers[rd] = processor.memory.load_half(
                    processor.registers[rs1]
                        .wrapping_add(offset.into())
                        .as_unsigned() as usize,
                ) as Self::RegisterType
            }
            Instruction::LW { rd, rs1, offset } => {
                processor.registers[rd] = processor.memory.load_word(
                    processor.registers[rs1]
                        .wrapping_add(offset.into())
                        .as_unsigned() as usize,
                )
            }
            Instruction::LBU { rd, rs1, offset } => {
                processor.registers[rd] = processor.memory.load_byte(
                    processor.registers[rs1]
                        .wrapping_add(offset.into())
                        .as_unsigned() as usize,
                ) as u8 as Self::RegisterType
            }
            Instruction::LHU { rd, rs1, offset } => {
                processor.registers[rd] = processor.memory.load_half(
                    processor.registers[rs1]
                        .wrapping_add(offset.into())
                        .as_unsigned() as usize,
                ) as u16 as Self::RegisterType
            }
            Instruction::SB { rs1, rs2, offset } => processor.memory.store_byte(
                processor.registers[rs1]
                    .wrapping_add(offset.into())
                    .as_unsigned() as usize,
                processor.registers[rs2] as i8,
            ),
            Instruction::SH { rs1, rs2, offset } => processor.memory.store_half(
                processor.registers[rs1]
                    .wrapping_add(offset.into())
                    .as_unsigned() as usize,
                processor.registers[rs2] as i16,
            ),
            Instruction::SW { rs1, rs2, offset } => processor.memory.store_word(
                processor.registers[rs1]
                    .wrapping_add(offset.into())
                    .as_unsigned() as usize,
                processor.registers[rs2],
            ),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::i12;
    use crate::memory::Memory;
    use crate::processor::Processor;
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
                let mut mem = Memory::default();
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
    macro_rules! csr_state {
        ($($index:literal: $value:expr),* $(,)?) => {
            {
                let csr = CSR32::default();
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
    macro_rules! processor_state {
        (
            registers: $register_state:tt
            $(, memory: $memory_state:tt)?
            $(, pc: $program_counter1:expr)?
            $(, csr: $csr:tt)?
            $(,)?
        ) => {
            Processor::<i32, CSR32> {
                registers: register_state!($register_state)
                $(, memory: memory_state!($memory_state))?
                $(, pc: $program_counter1)?
                $(, csrs: csr_state!($csr))?
                , ..Default::default()
            }
        };
        (
            {
                registers: $register_state:tt
                $(, memory: $memory_state:tt)?
                $(, pc: $program_counter2:expr)?
                $(, csr: $csr:tt)?
                $(,)?
            }
        ) => {
            processor_state!(
                registers: $register_state
                $(, memory: $memory_state)?
                $(, pc: $program_counter2)?
                $(, csr: $csr)?
            )
        };
    }
    macro_rules! test_execute {
        ($instruction:expr, changes: $initial_state:tt, to: $final_state:tt $(,)?) => {{
            // Arrange
            let mut processor = processor_state!($initial_state);

            // Act
            $instruction.execute(&mut processor).unwrap();

            // Assert
            let expected_state = processor_state!($final_state);
            assert_eq!(processor, expected_state);
            processor
        }};
    }
    macro_rules! test_execute_many {
        ($instructions:expr, changes: $initial_state:tt, to: $final_state:tt $(,)?) => {{
            // Arrange
            let mut processor = processor_state!($initial_state);

            // Act
            for instruction in $instructions {
                instruction.execute(&mut processor).unwrap();
            }

            // Assert
            let expected_state = processor_state!($final_state);
            assert_eq!(processor, expected_state);
            processor
        }};
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
    fn execute_snez() {
        test_execute_many!(
            Instruction::SNEZ(Register::A3, Register::A1),
            changes: {registers: {a1: -1}},
            to: {registers: {a1: -1, a3: 1}},
        );
        test_execute_many!(
            Instruction::SNEZ(Register::A3, Register::A1),
            changes: {registers: {a1: 0}},
            to: {registers: {a1: 0, a3: 0}},
        );
        test_execute_many!(
            Instruction::SNEZ(Register::A3, Register::A1),
            changes: {registers: {a1: 1}},
            to: {registers: {a1: 1, a3: 1}},
        );
    }

    #[test]
    fn execute_sltz() {
        test_execute_many!(
            Instruction::SLTZ(Register::A3, Register::A1),
            changes: {registers: {a1: -1}},
            to: {registers: {a1: -1, a3: 1}},
        );
        test_execute_many!(
            Instruction::SLTZ(Register::A3, Register::A1),
            changes: {registers: {a1: 0}},
            to: {registers: {a1: 0, a3: 0}},
        );
        test_execute_many!(
            Instruction::SLTZ(Register::A3, Register::A1),
            changes: {registers: {a1: 1}},
            to: {registers: {a1: 1, a3: 0}},
        );
    }

    #[test]
    fn execute_sgtz() {
        test_execute_many!(
            Instruction::SGLZ(Register::A3, Register::A1),
            changes: {registers: {a1: -1}},
            to: {registers: {a1: -1, a3: 0}},
        );
        test_execute_many!(
            Instruction::SGLZ(Register::A3, Register::A1),
            changes: {registers: {a1: 0}},
            to: {registers: {a1: 0, a3: 0}},
        );
        test_execute_many!(
            Instruction::SGLZ(Register::A3, Register::A1),
            changes: {registers: {a1: 1}},
            to: {registers: {a1: 1, a3: 1}},
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
    fn execute_sll() {
        test_execute!(
            Instruction::SLL{rd: Register::T4, rs1: Register::T5, rs2: Register::T6},
            changes: {registers: {t5: 0, t6: 4}},
            to: {registers: {t4: 0, t5: 0, t6: 4}},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S1, rs1: Register::S2, rs2: Register::S3},
            changes: {registers: {s1: 100, s2: 1, s3: 63}},
            to: {registers: {s1: i32::MIN, s2: 1, s3: 63}},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S4, rs1: Register::S5, rs2: Register::S6},
            changes: {registers: {s4: 100, s5: i32::MAX, s6: 1}},
            to: {registers: {s4: -2, s5: i32::MAX, s6: 1}},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S7, rs1: Register::S8, rs2: Register::S9},
            changes: {registers: {s7: 100, s8: 2 << 29, s9: 2}},
            to: {registers: {s7: 0, s8: 2 << 29, s9: 2}},
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
    fn execute_or() {
        test_execute!(
            Instruction::OR{rd: Register::S0, rs1: Register::S1, rs2: Register::T2},
            changes: {registers: {s1: 42, t2: -1}},
            to: {registers: {s0: -1, s1: 42, t2: -1}},
        );
        test_execute!(
            Instruction::OR{rd: Register::S2, rs1: Register::S3, rs2: Register::T3},
            changes: {registers: {s3: 2, t3: 2}},
            to: {registers: {s2: 2, s3: 2, t3: 2}},
        );
        test_execute!(
            Instruction::OR{rd: Register::S4, rs1: Register::S5, rs2: Register::T4},
            changes: {registers: {s5: 3, t4: 8}},
            to: {registers: {s4: 11, s5: 3, t4: 8}},
        );
        test_execute!(
            Instruction::OR{rd: Register::S4, rs1: Register::S5, rs2: Register::T5},
            changes: {registers: {s5: 19, t5: 7}},
            to: {registers: {s4: 23, s5: 19, t5: 7}},
        );
    }

    #[test]
    fn execute_srl() {
        test_execute!(
            Instruction::SRL{rd: Register::S10, rs1: Register::S11, rs2: Register::A0},
            changes: {registers: {s10: 100, s11: 64, a0: 5}},
            to: {registers: {s10: 2, s11: 64, a0: 5}},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A1, rs1: Register::A2, rs2: Register::A3},
            changes: {registers: {a1: 100, a2: -1, a3: 1}},
            to: {registers: {a1: i32::MAX, a2: -1, a3: 1}},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            changes: {registers: {a4: 100, a5: -1000, a6: 1}},
            to: {registers: {a4: 2147483148, a5: -1000, a6: 1}},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            changes: {registers: {a4: 100, a5: i32::MIN, a6: 63}},
            to: {registers: {a4: 1, a5: i32::MIN, a6: 63}},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A7, rs1: Register::A2, rs2: Register::A3},
            changes: {registers: {a7: 100, a2: 42, a3: 2}},
            to: {registers: {a7: 10, a2: 42, a3: 2}},
        );
    }

    #[test]
    fn execute_sra() {
        test_execute!(
            Instruction::SRA{rd: Register::T0, rs1: Register::T1, rs2: Register::T2},
            changes: {registers: {t0: 100, t1: 64, t2: 5}},
            to: {registers: {t0: 2, t1: 64, t2: 5}},
        );
        test_execute!(
            Instruction::SRA{rd: Register::T3, rs1: Register::T4, rs2: Register::T5},
            changes: {registers: {t3: 100, t4: 0, t5: 20}},
            to: {registers: {t3: 0, t4: 0, t5: 20}},
        );
        test_execute!(
            Instruction::SRA{rd: Register::T6, rs1: Register::S0, rs2: Register::TP},
            changes: {registers: {t6: 100, s0: -1, tp: 10}},
            to: {registers: {t6: -1, s0: -1, tp: 10}},
        );
        test_execute!(
            Instruction::SRA{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            changes: {registers: {a4: 100, a5: i32::MIN, a6: 63}},
            to: {registers: {a4: -1, a5: i32::MIN, a6: 63}},
        );
        test_execute!(
            Instruction::SRA{rd: Register::GP, rs1: Register::SP, rs2: Register::RA},
            changes: {registers: {gp: 100, sp: -1000, ra: 4}},
            to: {registers: {gp: -63, sp: -1000, ra: 4}},
        );
        test_execute!(
            Instruction::SRA{rd: Register::GP, rs1: Register::SP, rs2: Register::RA},
            changes: {registers: {gp: 100, sp: 42, ra: 2}},
            to: {registers: {gp: 10, sp: 42, ra: 2}},
        );
    }

    #[test]
    fn execute_xor() {
        test_execute!(
            Instruction::XOR{rd: Register::A0, rs1: Register::A1, rs2: Register::A2},
            changes: {registers: {a1: 42, a2: -1}},
            to: {registers: {a0: !(42), a1: 42, a2: -1}},
        );
        test_execute!(
            Instruction::XOR{rd: Register::A2, rs1: Register::A3, rs2: Register::A4},
            changes: {registers: {a3: 2, a4: 1}},
            to: {registers: {a2: 3, a3: 2, a4: 1}},
        );
    }

    #[test]
    fn execute_and() {
        test_execute!(
            Instruction::AND{rd: Register::S0, rs1: Register::S1, rs2: Register::A1},
            changes: {registers: {s1: 42, a1: -1}},
            to: {registers: {s0: 42, s1: 42, a1: -1}},
        );
        test_execute!(
            Instruction::AND{rd: Register::S2, rs1: Register::S3, rs2: Register::A2},
            changes: {registers: {s3: 2, a2: 2}},
            to: {registers: {s2: 2, s3: 2, a2: 2}},
        );
        test_execute!(
            Instruction::AND{rd: Register::S4, rs1: Register::S5, rs2: Register::A3},
            changes: {registers: {s5: 3, a3: 8}},
            to: {registers: {s4: 0, s5: 3, a3: 8}},
        );
        test_execute!(
            Instruction::AND{rd: Register::S4, rs1: Register::S5, rs2: Register::A4},
            changes: {registers: {s5: 19, a4: 7}},
            to: {registers: {s4: 3, s5: 19, a4: 7}},
        );
    }

    #[test]
    fn execute_lb() {
        test_execute!(
            Instruction::LB { rd: Register::T3, rs1: Register::T1, offset: 31, },
            changes: {registers: {t1: 0}, memory: {31: 21}},
            to: {registers: {t3: 21}, memory: {31: 21}},
        );
        test_execute!(
            Instruction::LB { rd: Register::T3, rs1: Register::T1, offset: 31, },
            changes: {registers: {t1:0}, memory: {31: -1}},
            to: {registers: {t3: -1}, memory: {31: -1}},
        );
    }

    #[test]
    fn execute_lh() {
        test_execute!(
            Instruction::LH { rd: Register::T3, rs1: Register::T1, offset: 21, },
            changes: {registers: {t1: 21}, memory: {42: 12}},
            to: {registers: {t1: 21, t3: 12}, memory: {42: 12}},
        );
        test_execute!(
            Instruction::LH { rd: Register::T3, rs1: Register::T1, offset: 21, },
            changes: {registers: {t1: 21}, memory: {42: -12}},
            to: {registers: {t1: 21, t3: -12}, memory: {42: -12}},
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

    #[test]
    fn execute_lbu() {
        test_execute!(
            Instruction::LBU { rd: Register::T3, rs1: Register::T1, offset: 31, },
            changes: {registers: {t1: 0}, memory: {31: 21}},
            to: {registers: {t3: 21}, memory: {31: 21}},
        );
        test_execute!(
            Instruction::LBU { rd: Register::T3, rs1: Register::T1, offset: 31, },
            changes: {registers: {t1:0}, memory: {31: -1}},
            to: {registers: {t3: u8::MAX as i32}, memory: {31: -1}},
        );
    }

    #[test]
    fn execute_lhu() {
        test_execute!(
            Instruction::LHU { rd: Register::T3, rs1: Register::T1, offset: 21, },
            changes: {registers: {t1: 21}, memory: {42: 12}},
            to: {registers: {t1: 21, t3: 12}, memory: {42: 12}},
        );
        test_execute!(
            Instruction::LHU { rd: Register::T3, rs1: Register::T1, offset: 21, },
            changes: {registers: {t1: 21}, memory: {42: -1}},
            to: {registers: {t1: 21, t3: u16::MAX as i32}, memory: {42: -1}},
        );
    }

    #[test]
    fn execute_sb() {
        test_execute!(
            Instruction::SB { rs1: Register::T1, rs2: Register::T3, offset: 0, },
            changes: {registers: {t1: 0, t3: 21}, memory: {4: 1}},
            to: {registers: {t3: 21}, memory: {0: 21, 4: 1}},
        );
        test_execute!(
            Instruction::SB { rs1: Register::T1, rs2: Register::T3, offset: 31, },
            changes: {registers: {t1:0, t3: -1}, memory: {32: 0}},
            to: {registers: {t3: -1}, memory: {31: -1, 32: 0}},
        );
    }

    #[test]
    fn execute_sh() {
        test_execute!(
            Instruction::SH { rs1: Register::T1, rs2: Register::T3, offset: 21, },
            changes: {registers: {t1: 21, t3: 12}, memory: {44: 1}},
            to: {registers: {t1: 21, t3: 12}, memory: {42: 12, 44: 1}},
        );
        test_execute!(
            Instruction::SH { rs1: Register::T1, rs2: Register::T3, offset: 21, },
            changes: {registers: {t1: 21, t3: -12}, memory: {44: 0}},
            to: {registers: {t1: 21, t3: -12}, memory: {42: -12, 44: 0}},
        );
    }

    #[test]
    fn execute_sw() {
        test_execute!(
            Instruction::SW {  rs1: Register::T1,rs2: Register::T3, offset: 31, },
            changes: {registers: {t1: 3, t3: 12}},
            to: {registers: {t1: 3, t3: 12}, memory: {34: 12}},
        );
    }

    #[test]
    fn execute_csrrw() {
        test_execute!(
            Instruction::CSRRW { rd: Register::A0, rs1: Register::S10, csr: 20 },
            changes: {registers: {a0: 3, s10: 12}},
            to: {registers: {a0: 0, s10: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRW { rd: Register::T1, rs1: Register::T0, csr: 5 },
            changes: {registers: {t1: 3, t0: 12}, csr: {5: 42}},
            to: {registers: {t1: 42, t0: 12}, csr: {5: 12}},
        );
    }

    #[test]
    fn execute_csrr() {
        test_execute_many!(
            Instruction::CSRR(Register::S3, 42),
            changes: {registers: {s3: 0}},
            to: {registers: {s3: 0}},
        );
        test_execute_many!(
            Instruction::CSRR(Register::S6, 42),
            changes: {registers: {s6: 3}, csr: {42: 42}},
            to: {registers: {s6: 42}, csr: {42: 42}},
        );
    }

    #[test]
    fn execute_csrw() {
        test_execute_many!(
            Instruction::CSRW(Register::S3, 42),
            changes: {registers: {s3: 0}},
            to: {registers: {s3: 0}},
        );
        test_execute_many!(
            Instruction::CSRW(Register::S6, 42),
            changes: {registers: {s6: 3}, csr: {42: 42}},
            to: {registers: {s6: 3}, csr: {42: 3}},
        );
    }

    #[test]
    fn execute_csrrs() {
        test_execute!(
            Instruction::CSRRS { rd: Register::T2, rs1: Register::S4, csr: 20 },
            changes: {registers: {t2: 3, s4: 12}},
            to: {registers: {t2: 0, s4: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::S8, rs1: Register::ZERO, csr: 20 },
            changes: {registers: {s8: 3}, csr: {20: 12}},
            to: {registers: {s8: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::A6, rs1: Register::S7, csr: 5 },
            changes: {registers: {a6: 3, s7: 0b10010010}, csr: {5: 0b10000101}},
            to: {registers: {a6: 0b10000101, s7: 0b10010010}, csr: {5: 0b10010111}},
        );
    }

    #[test]
    fn execute_csrrc() {
        test_execute!(
            Instruction::CSRRC { rd: Register::RA, rs1: Register::GP, csr: 20 },
            changes: {registers: {ra: 3, gp: 12}},
            to: {registers: {ra: 0, gp: 12}},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::S9, rs1: Register::ZERO, csr: 20 },
            changes: {registers: {s9: 30}, csr: {20: 12}},
            to: {registers: {s9: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRC { rd: Register::TP, rs1: Register::A7, csr: 5 },
            changes: {registers: {tp: 3, a7: 0b10010010}, csr: {5: 0b10000101}},
            to: {registers: {tp: 0b10000101, a7: 0b10010010}, csr: {5: 0b00000101}},
        );
    }

    #[test]
    fn execute_csrs() {
        test_execute_many!(
            Instruction::CSRS(Register::S3, 42),
            changes: {registers: {s3: 0}},
            to: {registers: {s3: 0}},
        );
        test_execute_many!(
            Instruction::CSRS(Register::S7, 5),
            changes: {registers: {s7: 0b10010010}, csr: {5: 0b10000101}},
            to: {registers: {s7: 0b10010010}, csr: {5: 0b10010111}},
        );
    }

    #[test]
    fn execute_csrc() {
        test_execute_many!(
            Instruction::CSRC(Register::S3, 42),
            changes: {registers: {s3: 0}},
            to: {registers: {s3: 0}},
        );
        test_execute_many!(
            Instruction::CSRC(Register::SP, 5),
            changes: {registers: {sp: 0b10010010}, csr: {5: 0b10000101}},
            to: {registers: {sp: 0b10010010}, csr: {5: 0b00000101}},
        );
    }

    #[test]
    fn execute_csrrwi() {
        test_execute!(
            Instruction::CSRRWI { rd: Register::A0, imm: 12, csr: 20 },
            changes: {registers: {a0: 3}},
            to: {registers: {a0: 0}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRWI { rd: Register::T1, imm: 32, csr: 5 },
            changes: {registers: {t1: 3}, csr: {5: 42}},
            to: {registers: {t1: 42}, csr: {5: 32}},
        );
    }

    #[test]
    fn execute_csrrsi() {
        test_execute!(
            Instruction::CSRRSI { rd: Register::T2, imm: 12, csr: 20 },
            changes: {registers: {t2: 3}},
            to: {registers: {t2: 0}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::S8, imm: 0, csr: 20 },
            changes: {registers: {s8: 3}, csr: {20: 12}},
            to: {registers: {s8: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::A6, imm: 0b01010, csr: 5 },
            changes: {registers: {a6: 3}, csr: {5: 0b10000101}},
            to: {registers: {a6: 0b10000101}, csr: {5: 0b10001111}},
        );
    }

    #[test]
    fn execute_csrrci() {
        test_execute!(
            Instruction::CSRRCI { rd: Register::RA, imm: 12, csr: 20 },
            changes: {registers: {ra: 3}},
            to: {registers: {ra: 0}},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::S9, imm: 0, csr: 20 },
            changes: {registers: {s9: 30}, csr: {20: 12}},
            to: {registers: {s9: 12}, csr: {20: 12}},
        );
        test_execute!(
            Instruction::CSRRCI { rd: Register::TP, imm: 0b10101, csr: 5 },
            changes: {registers: {tp: 3}, csr: {5: 0b10000101}},
            to: {registers: {tp: 0b10000101}, csr: {5: 0b10000000}},
        );
    }

    #[test]
    fn execute_csrwi() {
        test_execute_many!(
            Instruction::CSRWI(42, 0),
            changes: {registers: {}},
            to: {registers: {}},
        );
        test_execute_many!(
            Instruction::CSRWI(42, 5),
            changes: {registers: {}, csr: {42: 42}},
            to: {registers: {}, csr: {42: 5}},
        );
    }

    #[test]
    fn execute_csrsi() {
        test_execute_many!(
            Instruction::CSRSI(42, 0),
            changes: {registers: {}},
            to: {registers: {}},
        );
        test_execute_many!(
            Instruction::CSRSI(5, 0b10101),
            changes: {registers: {}, csr: {5: 0b10000101}},
            to: {registers: {}, csr: {5: 0b10010101}},
        );
    }

    #[test]
    fn execute_csrci() {
        test_execute_many!(
            Instruction::CSRCI(42, 0),
            changes: {registers: {}},
            to: {registers: {}},
        );
        test_execute_many!(
            Instruction::CSRCI(5, 0b10011),
            changes: {registers: {}, csr: {5: 0b10000101}},
            to: {registers: {}, csr: {5: 0b10000100}},
        );
    }
}
