//! The implementation of [crate::instruction_set::InstructionSet] for
//! [crate::instructions::Instruction].
use crate::csr::{ControlStatusRegisters, CSR32};
use crate::instruction_set::{Exception, InstructionSet};
use crate::integer::{AsSigned, AsUnsigned};
use crate::processor::Processor;
use crate::registers::Register;

use super::Instruction;

impl Instruction {
    /// A bit mask apply to a register before being used as the shift amount.
    const SHIFT_MASK: i32 = 0b_00000000_00000000_00000000_00011111;
}

impl InstructionSet for Instruction {
    type RegisterType = i32;
    type CSRType = CSR32;

    fn decode(raw_instruction: u32) -> Result<Self, Exception> {
        raw_instruction.try_into()
    }

    fn encode(self) -> u32 {
        self.encode()
    }

    fn execute(
        self,
        processor: &mut Processor<Self::RegisterType, Self::CSRType>,
    ) -> Result<(), Exception> {
        // By default, after this instruction, we will move to the next one. Instructions that do
        // something different e.g. JAL can set this variable to modify the pc.
        let mut pc = processor.pc + 4;

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
            Instruction::JAL { rd, offset } => {
                let jump = processor.pc + offset;
                if jump % 4 != 0 {
                    return Err(Exception::MisalignedInstructionFetch);
                }
                processor.registers[rd] = pc;
                pc = jump;
            }
            Instruction::JALR { rd, rs1, offset } => {
                let jump = (processor.registers[rs1] + offset as i32) & !1;
                if jump % 4 != 0 {
                    return Err(Exception::MisalignedInstructionFetch);
                }
                processor.registers[rd] = pc;
                pc = jump;
            }
            Instruction::BEQ { rs1, rs2, offset } => {
                if processor.registers[rs1] == processor.registers[rs2] {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
            Instruction::BNE { rs1, rs2, offset } => {
                if processor.registers[rs1] != processor.registers[rs2] {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
            Instruction::BLT { rs1, rs2, offset } => {
                if processor.registers[rs1] < processor.registers[rs2] {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
            Instruction::BGE { rs1, rs2, offset } => {
                if processor.registers[rs1] >= processor.registers[rs2] {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
            Instruction::BLTU { rs1, rs2, offset } => {
                if processor.registers[rs1].as_unsigned() < processor.registers[rs2].as_unsigned() {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
            Instruction::BGEU { rs1, rs2, offset } => {
                if processor.registers[rs1].as_unsigned() >= processor.registers[rs2].as_unsigned()
                {
                    if offset % 4 != 0 {
                        return Err(Exception::MisalignedInstructionFetch);
                    }
                    pc = processor.pc + offset as i32;
                }
            }
        }
        processor.pc = pc;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::i12;
    use crate::registers::Register;
    use crate::test::macros::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn execute_li() {
        for (i, pc_inc) in [
            (0, 4),
            (i12::MIN as i32, 4),
            (i12::MAX as i32, 4),
            (i12::MIN as i32 - 1, 8),
            (i12::MAX as i32 + 1, 8),
            (i32::MAX, 8),
            (i32::MIN, 8),
        ] {
            test_execute!(
                Instruction::LI(Register::T1, i),
                executed_on: {registers: {}},
                results_in: {registers: {t1: i}, pc: pc_inc},
            );
        }
    }

    #[test]
    fn execute_not() {
        test_execute!(
            Instruction::NOT(Register::A5, Register::A6),
            executed_on: {registers: {a6: -1}},
            results_in: {registers: {a5: 0, a6: -1}, pc: 4},
        );
        test_execute!(
            Instruction::NOT(Register::A5, Register::A6),
            executed_on: {registers: {a6: 0}},
            results_in: {registers: {a5: -1, a6: 0}, pc: 4},
        );
        test_execute!(
            Instruction::NOT(Register::A5, Register::A6),
            executed_on: {registers: {a6: 42}},
            results_in: {registers: {a5: -43, a6: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_neg() {
        test_execute!(
            Instruction::NEG(Register::S5, Register::S6),
            executed_on: {registers: {s6: -1}},
            results_in: {registers: {s5: 1, s6: -1}, pc: 4},
        );
        test_execute!(
            Instruction::NEG(Register::S5, Register::S6),
            executed_on: {registers: {s6: -1}},
            results_in: {registers: {s5: 1, s6: -1}, pc: 4},
        );
        test_execute!(
            Instruction::NEG(Register::S5, Register::S6),
            executed_on: {registers: {s6: 42}},
            results_in: {registers: {s5: -42, s6: 42}, pc: 4},
        );
        test_execute!(
            Instruction::NEG(Register::S5, Register::S6),
            executed_on: {registers: {s6: 0}},
            results_in: {registers: {s5: 0, s6: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_mov() {
        test_execute!(
            Instruction::MOV(Register::T5, Register::T6),
            executed_on: {registers: {t6: -1}},
            results_in: {registers: {t5: -1, t6: -1}, pc: 4},
        );
    }

    #[test]
    fn execute_seqz() {
        test_execute!(
            Instruction::SEQZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: -1}},
            results_in: {registers: {a1: -1, a3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SEQZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 0}},
            results_in: {registers: {a1: 0, a3: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SEQZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 1}},
            results_in: {registers: {a1: 1, a3: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_snez() {
        test_execute!(
            Instruction::SNEZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: -1}},
            results_in: {registers: {a1: -1, a3: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SNEZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 0}},
            results_in: {registers: {a1: 0, a3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SNEZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 1}},
            results_in: {registers: {a1: 1, a3: 1}, pc: 4},
        );
    }

    #[test]
    fn execute_sltz() {
        test_execute!(
            Instruction::SLTZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: -1}},
            results_in: {registers: {a1: -1, a3: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLTZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 0}},
            results_in: {registers: {a1: 0, a3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SLTZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 1}},
            results_in: {registers: {a1: 1, a3: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_sgtz() {
        test_execute!(
            Instruction::SGLZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: -1}},
            results_in: {registers: {a1: -1, a3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SGLZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 0}},
            results_in: {registers: {a1: 0, a3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SGLZ(Register::A3, Register::A1),
            executed_on: {registers: {a1: 1}},
            results_in: {registers: {a1: 1, a3: 1}, pc: 4},
        );
    }

    #[test]
    fn execute_nop() {
        test_execute!(
            Instruction::NOP,
            executed_on: {registers: {}},
            results_in: {registers: {}, pc: 4},
        );
    }

    #[test]
    fn execute_lui() {
        test_execute!(
            Instruction::LUI { rd: Register::S11, imm: 0x2BAAA },
            executed_on: {registers: {}},
            results_in: {registers: {s11: 0x2BAA_A000}, pc: 4},
        );
        test_execute!(
            Instruction::LUI { rd: Register::S11, imm: 0xDEAD_B },
            executed_on: {registers: {}},
            results_in: {registers: {s11: i32::from_be_bytes([0xDE, 0xAD, 0xB0, 0x00])}, pc: 4},
        );
    }

    #[test]
    fn execute_auipc() {
        test_execute!(
            Instruction::AUIPC { rd: Register::SP, imm: 0x2BAAA },
            executed_on: {registers: {}, pc: 0x0000_0AAD},
            results_in: {registers: {sp: 0x2BAA_AAAD}, pc: 0x0000_0AB1},
        );
        test_execute!(
            Instruction::AUIPC { rd: Register::SP, imm: 0xDEAD_B },
            executed_on: {registers: {}, pc: 0x0000_0EAF},
            results_in: {registers: {sp: i32::from_be_bytes([0xDE, 0xAD, 0xBE, 0xAF])}, pc: 0x0000_0EB3},
        );
    }

    #[test]
    fn execute_addi() {
        test_execute!(
            Instruction::ADDI{rd: Register::T4, rs1: Register::T1, imm: 42},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 84}, pc: 4},
        );
    }

    #[test]
    fn execute_slti() {
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: 43},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: 42},
            executed_on: {registers: {t1: 42, t4: 100}},
            results_in: {registers: {t1: 42, t4: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SLTI{rd: Register::T4, rs1: Register::T1, imm: -43},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_sltiu() {
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 43},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 42},
            executed_on: {registers: {t1: 42, t4: 100}},
            results_in: {registers: {t1: 42, t4: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: -43},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 1},
            executed_on: {registers: {t1: 42}},
            results_in: {registers: {t1: 42, t4: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SLTIU{rd: Register::T4, rs1: Register::T1, imm: 1},
            executed_on: {registers: {t1: 0}},
            results_in: {registers: {t1: 0, t4: 1}, pc: 4},
        );
    }

    #[test]
    fn execute_xori() {
        test_execute!(
            Instruction::XORI{rd: Register::A0, rs1: Register::A1, imm: -1},
            executed_on: {registers: {a1: 42}},
            results_in: {registers: {a0: !(42), a1: 42}, pc: 4},
        );
        test_execute!(
            Instruction::XORI{rd: Register::A2, rs1: Register::A3, imm: 1},
            executed_on: {registers: {a3: 2}},
            results_in: {registers: {a2: 3, a3: 2}, pc: 4},
        );
    }

    #[test]
    fn execute_ori() {
        test_execute!(
            Instruction::ORI{rd: Register::S0, rs1: Register::S1, imm: -1},
            executed_on: {registers: {s1: 42}},
            results_in: {registers: {s0: -1, s1: 42}, pc: 4},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S2, rs1: Register::S3, imm: 2},
            executed_on: {registers: {s3: 2}},
            results_in: {registers: {s2: 2, s3: 2}, pc: 4},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S4, rs1: Register::S5, imm: 8},
            executed_on: {registers: {s5: 3}},
            results_in: {registers: {s4: 11, s5: 3}, pc: 4},
        );
        test_execute!(
            Instruction::ORI{rd: Register::S4, rs1: Register::S5, imm: 7},
            executed_on: {registers: {s5: 19}},
            results_in: {registers: {s4: 23, s5: 19}, pc: 4},
        );
    }

    #[test]
    fn execute_andi() {
        test_execute!(
            Instruction::ANDI{rd: Register::S0, rs1: Register::S1, imm: -1},
            executed_on: {registers: {s1: 42}},
            results_in: {registers: {s0: 42, s1: 42}, pc: 4},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S2, rs1: Register::S3, imm: 2},
            executed_on: {registers: {s3: 2}},
            results_in: {registers: {s2: 2, s3: 2}, pc: 4},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S4, rs1: Register::S5, imm: 8},
            executed_on: {registers: {s5: 3}},
            results_in: {registers: {s4: 0, s5: 3}, pc: 4},
        );
        test_execute!(
            Instruction::ANDI{rd: Register::S4, rs1: Register::S5, imm: 7},
            executed_on: {registers: {s5: 19}},
            results_in: {registers: {s4: 3, s5: 19}, pc: 4},
        );
    }

    #[test]
    fn execute_slli() {
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            executed_on: {registers: {ra: 1}},
            results_in: {registers: {sp: 32, ra: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            executed_on: {registers: {ra: 0}},
            results_in: {registers: {sp: 0, ra: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            executed_on: {registers: {ra: i32::MAX}},
            results_in: {registers: {sp: -2, ra: i32::MAX}, pc: 4},
        );
        test_execute!(
            Instruction::SLLI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            executed_on: {registers: {ra: 2 << 29}},
            results_in: {registers: {sp: 0, ra: 2 << 29}, pc: 4},
        );
    }

    #[test]
    fn execute_srli() {
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            executed_on: {registers: {ra: 64}},
            results_in: {registers: {sp: 2, ra: 64}, pc: 4},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            executed_on: {registers: {ra: 0}},
            results_in: {registers: {sp: 0, ra: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            executed_on: {registers: {ra: -1}},
            results_in: {registers: {sp: i32::MAX, ra: -1}, pc: 4},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 1},
            executed_on: {registers: {ra: -1000}},
            results_in: {registers: {sp: 2147483148, ra: -1000}, pc: 4},
        );
        test_execute!(
            Instruction::SRLI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            executed_on: {registers: {ra: 42}},
            results_in: {registers: {sp: 10, ra: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_srai() {
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 5},
            executed_on: {registers: {ra: 64}},
            results_in: {registers: {sp: 2, ra: 64}, pc: 4},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 20},
            executed_on: {registers: {ra: 0}},
            results_in: {registers: {sp: 0, ra: 0}, pc: 4},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 10},
            executed_on: {registers: {ra: -1}},
            results_in: {registers: {sp: -1, ra: -1}, pc: 4},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 4},
            executed_on: {registers: {ra: -1000}},
            results_in: {registers: {sp: -63, ra: -1000}, pc: 4},
        );
        test_execute!(
            Instruction::SRAI{rd: Register::SP, rs1: Register::RA, shamt: 2},
            executed_on: {registers: {ra: 42}},
            results_in: {registers: {sp: 10, ra: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_add() {
        test_execute!(
            Instruction::ADD{rd: Register::T3, rs1: Register::T1, rs2: Register::T2},
            executed_on: {registers: {t1: 21, t2: 21}},
            results_in: {registers: {t1: 21, t2: 21, t3: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_sub() {
        test_execute!(
            Instruction::SUB { rd: Register::T3, rs1: Register::T1, rs2: Register::T2, },
            executed_on: {registers: {t1: 45, t2: 3}},
            results_in: {registers: {t1: 45, t2: 3, t3: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_sll() {
        test_execute!(
            Instruction::SLL{rd: Register::T4, rs1: Register::T5, rs2: Register::T6},
            executed_on: {registers: {t5: 0, t6: 4}},
            results_in: {registers: {t4: 0, t5: 0, t6: 4}, pc: 4},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S1, rs1: Register::S2, rs2: Register::S3},
            executed_on: {registers: {s1: 100, s2: 1, s3: 63}},
            results_in: {registers: {s1: i32::MIN, s2: 1, s3: 63}, pc: 4},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S4, rs1: Register::S5, rs2: Register::S6},
            executed_on: {registers: {s4: 100, s5: i32::MAX, s6: 1}},
            results_in: {registers: {s4: -2, s5: i32::MAX, s6: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLL{rd: Register::S7, rs1: Register::S8, rs2: Register::S9},
            executed_on: {registers: {s7: 100, s8: 2 << 29, s9: 2}},
            results_in: {registers: {s7: 0, s8: 2 << 29, s9: 2}, pc: 4},
        );
    }

    #[test]
    fn execute_slt() {
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            executed_on: {registers: {t1: 42, t3: 43}},
            results_in: {registers: {t1: 42, t4: 1, t3: 43}, pc: 4},
        );
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            executed_on: {registers: {t1: 42, t4: 100, t3: 42}},
            results_in: {registers: {t1: 42, t4: 0, t3: 42}, pc: 4},
        );
        test_execute!(
            Instruction::SLT{rd: Register::T4, rs1: Register::T1, rs2: Register::T3},
            executed_on: {registers: {t1: 42, t3: -43}},
            results_in: {registers: {t1: 42, t4: 0, t3: -43}, pc: 4},
        );
    }

    #[test]
    fn execute_sltu() {
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            executed_on: {registers: {t1: 42, a4: 43}},
            results_in: {registers: {t1: 42, t4: 1, a4: 43}, pc: 4},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            executed_on: {registers: {t1: 42, t4: 100, a4: 42}},
            results_in: {registers: {t1: 42, t4: 0, a4: 42}, pc: 4},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            executed_on: {registers: {t1: 42, a4: -43}},
            results_in: {registers: {t1: 42, t4: 1, a4: -43}, pc: 4},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            executed_on: {registers: {t1: 42, a4: 1}},
            results_in: {registers: {t1: 42, t4: 0, a4: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SLTU{rd: Register::T4, rs1: Register::T1, rs2: Register::A4},
            executed_on: {registers: {t1: 0, a4: 1}},
            results_in: {registers: {t1: 0, t4: 1, a4: 1}, pc: 4},
        );
    }

    #[test]
    fn execute_or() {
        test_execute!(
            Instruction::OR{rd: Register::S0, rs1: Register::S1, rs2: Register::T2},
            executed_on: {registers: {s1: 42, t2: -1}},
            results_in: {registers: {s0: -1, s1: 42, t2: -1}, pc: 4},
        );
        test_execute!(
            Instruction::OR{rd: Register::S2, rs1: Register::S3, rs2: Register::T3},
            executed_on: {registers: {s3: 2, t3: 2}},
            results_in: {registers: {s2: 2, s3: 2, t3: 2}, pc: 4},
        );
        test_execute!(
            Instruction::OR{rd: Register::S4, rs1: Register::S5, rs2: Register::T4},
            executed_on: {registers: {s5: 3, t4: 8}},
            results_in: {registers: {s4: 11, s5: 3, t4: 8}, pc: 4},
        );
        test_execute!(
            Instruction::OR{rd: Register::S4, rs1: Register::S5, rs2: Register::T5},
            executed_on: {registers: {s5: 19, t5: 7}},
            results_in: {registers: {s4: 23, s5: 19, t5: 7}, pc: 4},
        );
    }

    #[test]
    fn execute_srl() {
        test_execute!(
            Instruction::SRL{rd: Register::S10, rs1: Register::S11, rs2: Register::A0},
            executed_on: {registers: {s10: 100, s11: 64, a0: 5}},
            results_in: {registers: {s10: 2, s11: 64, a0: 5}, pc: 4},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A1, rs1: Register::A2, rs2: Register::A3},
            executed_on: {registers: {a1: 100, a2: -1, a3: 1}},
            results_in: {registers: {a1: i32::MAX, a2: -1, a3: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            executed_on: {registers: {a4: 100, a5: -1000, a6: 1}},
            results_in: {registers: {a4: 2147483148, a5: -1000, a6: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            executed_on: {registers: {a4: 100, a5: i32::MIN, a6: 63}},
            results_in: {registers: {a4: 1, a5: i32::MIN, a6: 63}, pc: 4},
        );
        test_execute!(
            Instruction::SRL{rd: Register::A7, rs1: Register::A2, rs2: Register::A3},
            executed_on: {registers: {a7: 100, a2: 42, a3: 2}},
            results_in: {registers: {a7: 10, a2: 42, a3: 2}, pc: 4},
        );
    }

    #[test]
    fn execute_sra() {
        test_execute!(
            Instruction::SRA{rd: Register::T0, rs1: Register::T1, rs2: Register::T2},
            executed_on: {registers: {t0: 100, t1: 64, t2: 5}},
            results_in: {registers: {t0: 2, t1: 64, t2: 5}, pc: 4},
        );
        test_execute!(
            Instruction::SRA{rd: Register::T3, rs1: Register::T4, rs2: Register::T5},
            executed_on: {registers: {t3: 100, t4: 0, t5: 20}},
            results_in: {registers: {t3: 0, t4: 0, t5: 20}, pc: 4},
        );
        test_execute!(
            Instruction::SRA{rd: Register::T6, rs1: Register::S0, rs2: Register::TP},
            executed_on: {registers: {t6: 100, s0: -1, tp: 10}},
            results_in: {registers: {t6: -1, s0: -1, tp: 10}, pc: 4},
        );
        test_execute!(
            Instruction::SRA{rd: Register::A4, rs1: Register::A5, rs2: Register::A6},
            executed_on: {registers: {a4: 100, a5: i32::MIN, a6: 63}},
            results_in: {registers: {a4: -1, a5: i32::MIN, a6: 63}, pc: 4},
        );
        test_execute!(
            Instruction::SRA{rd: Register::GP, rs1: Register::SP, rs2: Register::RA},
            executed_on: {registers: {gp: 100, sp: -1000, ra: 4}},
            results_in: {registers: {gp: -63, sp: -1000, ra: 4}, pc: 4},
        );
        test_execute!(
            Instruction::SRA{rd: Register::GP, rs1: Register::SP, rs2: Register::RA},
            executed_on: {registers: {gp: 100, sp: 42, ra: 2}},
            results_in: {registers: {gp: 10, sp: 42, ra: 2}, pc: 4},
        );
    }

    #[test]
    fn execute_xor() {
        test_execute!(
            Instruction::XOR{rd: Register::A0, rs1: Register::A1, rs2: Register::A2},
            executed_on: {registers: {a1: 42, a2: -1}},
            results_in: {registers: {a0: !(42), a1: 42, a2: -1}, pc: 4},
        );
        test_execute!(
            Instruction::XOR{rd: Register::A2, rs1: Register::A3, rs2: Register::A4},
            executed_on: {registers: {a3: 2, a4: 1}},
            results_in: {registers: {a2: 3, a3: 2, a4: 1}, pc: 4},
        );
    }

    #[test]
    fn execute_and() {
        test_execute!(
            Instruction::AND{rd: Register::S0, rs1: Register::S1, rs2: Register::A1},
            executed_on: {registers: {s1: 42, a1: -1}},
            results_in: {registers: {s0: 42, s1: 42, a1: -1}, pc: 4},
        );
        test_execute!(
            Instruction::AND{rd: Register::S2, rs1: Register::S3, rs2: Register::A2},
            executed_on: {registers: {s3: 2, a2: 2}},
            results_in: {registers: {s2: 2, s3: 2, a2: 2}, pc: 4},
        );
        test_execute!(
            Instruction::AND{rd: Register::S4, rs1: Register::S5, rs2: Register::A3},
            executed_on: {registers: {s5: 3, a3: 8}},
            results_in: {registers: {s4: 0, s5: 3, a3: 8}, pc: 4},
        );
        test_execute!(
            Instruction::AND{rd: Register::S4, rs1: Register::S5, rs2: Register::A4},
            executed_on: {registers: {s5: 19, a4: 7}},
            results_in: {registers: {s4: 3, s5: 19, a4: 7}, pc: 4},
        );
    }

    #[test]
    fn execute_lb() {
        test_execute!(
            Instruction::LB { rd: Register::T3, rs1: Register::T1, offset: 31, },
            executed_on: {registers: {t1: 0}, memory: {31: 21}},
            results_in: {registers: {t3: 21}, memory: {31: 21}, pc: 4},
        );
        test_execute!(
            Instruction::LB { rd: Register::T3, rs1: Register::T1, offset: 31, },
            executed_on: {registers: {t1:0}, memory: {31: -1}},
            results_in: {registers: {t3: -1}, memory: {31: -1}, pc: 4},
        );
    }

    #[test]
    fn execute_lh() {
        test_execute!(
            Instruction::LH { rd: Register::T3, rs1: Register::T1, offset: 21, },
            executed_on: {registers: {t1: 21}, memory: {42: 12}},
            results_in: {registers: {t1: 21, t3: 12}, memory: {42: 12}, pc: 4},
        );
        test_execute!(
            Instruction::LH { rd: Register::T3, rs1: Register::T1, offset: 21, },
            executed_on: {registers: {t1: 21}, memory: {42: -12}},
            results_in: {registers: {t1: 21, t3: -12}, memory: {42: -12}, pc: 4},
        );
    }

    #[test]
    fn execute_lw() {
        test_execute!(
            Instruction::LW { rd: Register::T3, rs1: Register::T1, offset: 31, },
            executed_on: {registers: {t1: 3}, memory: {34: 12}},
            results_in: {registers: {t1: 3, t3: 12}, memory: {34: 12}, pc: 4},
        );
    }

    #[test]
    fn execute_lbu() {
        test_execute!(
            Instruction::LBU { rd: Register::T3, rs1: Register::T1, offset: 31, },
            executed_on: {registers: {t1: 0}, memory: {31: 21}},
            results_in: {registers: {t3: 21}, memory: {31: 21}, pc: 4},
        );
        test_execute!(
            Instruction::LBU { rd: Register::T3, rs1: Register::T1, offset: 31, },
            executed_on: {registers: {t1:0}, memory: {31: -1}},
            results_in: {registers: {t3: u8::MAX as i32}, memory: {31: -1}, pc: 4},
        );
    }

    #[test]
    fn execute_lhu() {
        test_execute!(
            Instruction::LHU { rd: Register::T3, rs1: Register::T1, offset: 21, },
            executed_on: {registers: {t1: 21}, memory: {42: 12}},
            results_in: {registers: {t1: 21, t3: 12}, memory: {42: 12}, pc: 4},
        );
        test_execute!(
            Instruction::LHU { rd: Register::T3, rs1: Register::T1, offset: 21, },
            executed_on: {registers: {t1: 21}, memory: {42: -1}},
            results_in: {registers: {t1: 21, t3: u16::MAX as i32}, memory: {42: -1}, pc: 4},
        );
    }

    #[test]
    fn execute_sb() {
        test_execute!(
            Instruction::SB { rs1: Register::T1, rs2: Register::T3, offset: 0, },
            executed_on: {registers: {t1: 0, t3: 21}, memory: {4: 1}},
            results_in: {registers: {t3: 21}, memory: {0: 21, 4: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SB { rs1: Register::T1, rs2: Register::T3, offset: 31, },
            executed_on: {registers: {t1:0, t3: -1}, memory: {32: 0}},
            results_in: {registers: {t3: -1}, memory: {31: -1, 32: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_sh() {
        test_execute!(
            Instruction::SH { rs1: Register::T1, rs2: Register::T3, offset: 21, },
            executed_on: {registers: {t1: 21, t3: 12}, memory: {44: 1}},
            results_in: {registers: {t1: 21, t3: 12}, memory: {42: 12, 44: 1}, pc: 4},
        );
        test_execute!(
            Instruction::SH { rs1: Register::T1, rs2: Register::T3, offset: 21, },
            executed_on: {registers: {t1: 21, t3: -12}, memory: {44: 0}},
            results_in: {registers: {t1: 21, t3: -12}, memory: {42: -12, 44: 0}, pc: 4},
        );
    }

    #[test]
    fn execute_sw() {
        test_execute!(
            Instruction::SW {  rs1: Register::T1,rs2: Register::T3, offset: 31, },
            executed_on: {registers: {t1: 3, t3: 12}},
            results_in: {registers: {t1: 3, t3: 12}, memory: {34: 12}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrw() {
        test_execute!(
            Instruction::CSRRW { rd: Register::A0, rs1: Register::S10, csr: 20 },
            executed_on: {registers: {a0: 3, s10: 12}},
            results_in: {registers: {a0: 0, s10: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRW { rd: Register::T1, rs1: Register::T0, csr: 5 },
            executed_on: {registers: {t1: 3, t0: 12}, csr: {5: 42}},
            results_in: {registers: {t1: 42, t0: 12}, csr: {5: 12}, pc: 4},
        );
    }

    #[test]
    fn execute_csrr() {
        test_execute!(
            Instruction::CSRR(Register::S3, 42),
            executed_on: {registers: {s3: 0}},
            results_in: {registers: {s3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::CSRR(Register::S6, 42),
            executed_on: {registers: {s6: 3}, csr: {42: 42}},
            results_in: {registers: {s6: 42}, csr: {42: 42}, pc: 4},
        );
    }

    #[test]
    fn execute_csrw() {
        test_execute!(
            Instruction::CSRW(Register::S3, 42),
            executed_on: {registers: {s3: 0}},
            results_in: {registers: {s3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::CSRW(Register::S6, 42),
            executed_on: {registers: {s6: 3}, csr: {42: 42}},
            results_in: {registers: {s6: 3}, csr: {42: 3}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrs() {
        test_execute!(
            Instruction::CSRRS { rd: Register::T2, rs1: Register::S4, csr: 20 },
            executed_on: {registers: {t2: 3, s4: 12}},
            results_in: {registers: {t2: 0, s4: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::S8, rs1: Register::ZERO, csr: 20 },
            executed_on: {registers: {s8: 3}, csr: {20: 12}},
            results_in: {registers: {s8: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::A6, rs1: Register::S7, csr: 5 },
            executed_on: {registers: {a6: 3, s7: 0b10010010}, csr: {5: 0b10000101}},
            results_in: {registers: {a6: 0b10000101, s7: 0b10010010}, csr: {5: 0b10010111}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrc() {
        test_execute!(
            Instruction::CSRRC { rd: Register::RA, rs1: Register::GP, csr: 20 },
            executed_on: {registers: {ra: 3, gp: 12}},
            results_in: {registers: {ra: 0, gp: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRS { rd: Register::S9, rs1: Register::ZERO, csr: 20 },
            executed_on: {registers: {s9: 30}, csr: {20: 12}},
            results_in: {registers: {s9: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRC { rd: Register::TP, rs1: Register::A7, csr: 5 },
            executed_on: {registers: {tp: 3, a7: 0b10010010}, csr: {5: 0b10000101}},
            results_in: {registers: {tp: 0b10000101, a7: 0b10010010}, csr: {5: 0b00000101}, pc: 4},
        );
    }

    #[test]
    fn execute_csrs() {
        test_execute!(
            Instruction::CSRS(Register::S3, 42),
            executed_on: {registers: {s3: 0}},
            results_in: {registers: {s3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::CSRS(Register::S7, 5),
            executed_on: {registers: {s7: 0b10010010}, csr: {5: 0b10000101}},
            results_in: {registers: {s7: 0b10010010}, csr: {5: 0b10010111}, pc: 4},
        );
    }

    #[test]
    fn execute_csrc() {
        test_execute!(
            Instruction::CSRC(Register::S3, 42),
            executed_on: {registers: {s3: 0}},
            results_in: {registers: {s3: 0}, pc: 4},
        );
        test_execute!(
            Instruction::CSRC(Register::SP, 5),
            executed_on: {registers: {sp: 0b10010010}, csr: {5: 0b10000101}},
            results_in: {registers: {sp: 0b10010010}, csr: {5: 0b00000101}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrwi() {
        test_execute!(
            Instruction::CSRRWI { rd: Register::A0, imm: 12, csr: 20 },
            executed_on: {registers: {a0: 3}},
            results_in: {registers: {a0: 0}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRWI { rd: Register::T1, imm: 32, csr: 5 },
            executed_on: {registers: {t1: 3}, csr: {5: 42}},
            results_in: {registers: {t1: 42}, csr: {5: 32}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrsi() {
        test_execute!(
            Instruction::CSRRSI { rd: Register::T2, imm: 12, csr: 20 },
            executed_on: {registers: {t2: 3}},
            results_in: {registers: {t2: 0}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::S8, imm: 0, csr: 20 },
            executed_on: {registers: {s8: 3}, csr: {20: 12}},
            results_in: {registers: {s8: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::A6, imm: 0b01010, csr: 5 },
            executed_on: {registers: {a6: 3}, csr: {5: 0b10000101}},
            results_in: {registers: {a6: 0b10000101}, csr: {5: 0b10001111}, pc: 4},
        );
    }

    #[test]
    fn execute_csrrci() {
        test_execute!(
            Instruction::CSRRCI { rd: Register::RA, imm: 12, csr: 20 },
            executed_on: {registers: {ra: 3}},
            results_in: {registers: {ra: 0}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRSI { rd: Register::S9, imm: 0, csr: 20 },
            executed_on: {registers: {s9: 30}, csr: {20: 12}},
            results_in: {registers: {s9: 12}, csr: {20: 12}, pc: 4},
        );
        test_execute!(
            Instruction::CSRRCI { rd: Register::TP, imm: 0b10101, csr: 5 },
            executed_on: {registers: {tp: 3}, csr: {5: 0b10000101}},
            results_in: {registers: {tp: 0b10000101}, csr: {5: 0b10000000}, pc: 4},
        );
    }

    #[test]
    fn execute_csrwi() {
        test_execute!(
            Instruction::CSRWI(42, 0),
            executed_on: {registers: {}},
            results_in: {registers: {}, pc: 4},
        );
        test_execute!(
            Instruction::CSRWI(42, 5),
            executed_on: {registers: {}, csr: {42: 42}},
            results_in: {registers: {}, csr: {42: 5}, pc: 4},
        );
    }

    #[test]
    fn execute_csrsi() {
        test_execute!(
            Instruction::CSRSI(42, 0),
            executed_on: {registers: {}},
            results_in: {registers: {}, pc: 4},
        );
        test_execute!(
            Instruction::CSRSI(5, 0b10101),
            executed_on: {registers: {}, csr: {5: 0b10000101}},
            results_in: {registers: {}, csr: {5: 0b10010101}, pc: 4},
        );
    }

    #[test]
    fn execute_csrci() {
        test_execute!(
            Instruction::CSRCI(42, 0),
            executed_on: {registers: {}},
            results_in: {registers: {}, pc: 4},
        );
        test_execute!(
            Instruction::CSRCI(5, 0b10011),
            executed_on: {registers: {}, csr: {5: 0b10000101}},
            results_in: {registers: {}, csr: {5: 0b10000100}, pc: 4},
        );
    }

    #[test]
    fn execute_jal() {
        test_execute!(
            Instruction::JAL { rd: Register::RA, offset: 84 },
            executed_on: {registers: {}},
            results_in: {registers: {ra: 4}, pc: 84},
        );
        test_execute!(
            Instruction::JAL { rd: Register::ZERO, offset: 5000 },
            executed_on: {registers: {}, pc: 5000},
            results_in: {registers: {}, pc: 10000 },
        );
        test_execute!(
            Instruction::JAL { rd: Register::T0, offset: -15000 },
            executed_on: {registers: {}, pc: 40000},
            results_in: {registers: {t0: 40004}, pc: 25000 },
        );
        test_execute!(
            Instruction::JAL { rd: Register::RA, offset: -42 },
            executed_on: {registers: {}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_jal_pseudoinstruction() {
        test_execute!(
            Instruction::JAL(84),
            executed_on: {registers: {}},
            results_in: {registers: {ra: 4}, pc: 84},
        );
        test_execute!(
            Instruction::JAL(1000),
            executed_on: {registers: {}, pc: 5000},
            results_in: {registers: {ra: 5004}, pc: 6000 },
        );
        test_execute!(
            Instruction::JAL(-42),
            executed_on: {registers: {}, pc: 82},
            results_in: {registers: {ra: 86}, pc: 40 },
        );
        test_execute!(
            Instruction::JAL(-42),
            executed_on: {registers: {}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_j() {
        test_execute!(
            Instruction::J(84),
            executed_on: {registers: {}},
            results_in: {registers: {}, pc: 84},
        );
        test_execute!(
            Instruction::J(1000),
            executed_on: {registers: {}, pc: 5000},
            results_in: {registers: {}, pc: 6000 },
        );
        test_execute!(
            Instruction::J(-42),
            executed_on: {registers: {}, pc: 82},
            results_in: {registers: {}, pc: 40 },
        );
        test_execute!(
            Instruction::J(-42),
            executed_on: {registers: {}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_jalr() {
        test_execute!(
            Instruction::JALR { rd: Register::RA, rs1: Register::ZERO, offset: 84 },
            executed_on: {registers: {}},
            results_in: {registers: {ra: 4}, pc: 84},
        );
        test_execute!(
            Instruction::JALR { rd: Register::ZERO, rs1: Register::A0, offset: 5000 },
            executed_on: {registers: {a0: 5000}, pc: 5000},
            results_in: {registers: {a0: 5000}, pc: 10000 },
        );
        test_execute!(
            Instruction::JALR { rd: Register::ZERO, rs1: Register::A0, offset: 5000 },
            executed_on: {registers: {a0: 5005}, pc: 5000},
            results_in: {registers: {a0: 5005}, pc: 10004 },
        );
        test_execute!(
            Instruction::JALR { rd: Register::T0, rs1: Register::T1, offset: -15000 },
            executed_on: {registers: {t1: 40000}, pc: 4},
            results_in: {registers: {t0: 8, t1: 40000}, pc: 25000 },
        );
        test_execute!(
            Instruction::JALR { rd: Register::RA, rs1: Register::S3, offset: -42 },
            executed_on: {registers: {s3: 84}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_jalr_psuedoinstruction() {
        test_execute!(
            Instruction::JALR(Register::A0),
            executed_on: {registers: {a0: 5000}, pc: 1000},
            results_in: {registers: {ra: 1004, a0: 5000}, pc: 5000 },
        );
        test_execute!(
            Instruction::JALR(Register::S3),
            executed_on: {registers: {s3: 42}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_jr() {
        test_execute!(
            Instruction::JR(Register::A0),
            executed_on: {registers: {a0: 5000}, pc: 1000},
            results_in: {registers: {a0: 5000}, pc: 5000 },
        );
        test_execute!(
            Instruction::JR(Register::S3),
            executed_on: {registers: {s3: 42}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_ret() {
        test_execute!(
            Instruction::RET,
            executed_on: {registers: {ra: 5000}, pc: 1000},
            results_in: {registers: {ra: 5000}, pc: 5000 },
        );
        test_execute!(
            Instruction::RET,
            executed_on: {registers: {ra: 42}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
        );
    }

    #[test]
    fn execute_call() {
        test_execute!(
            Instruction::CALL(50_000_000),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {ra: 1008}, pc: 50_001_000 },
        );
        test_execute!(
            Instruction::CALL(50_000_003),
            executed_on: {registers: {}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
            with_final_state: {registers: {ra: 49999956}, pc: 88},
        );
    }

    #[test]
    fn execute_tail() {
        test_execute!(
            Instruction::TAIL(50_000_000),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {t1: 50_000_872}, pc: 50_001_000 },
        );
        test_execute!(
            Instruction::TAIL(50_000_003),
            executed_on: {registers: {}, pc: 84},
            throws: Exception::MisalignedInstructionFetch,
            with_final_state: {registers: {t1: 49999956}, pc: 88},
        );
    }

    #[test]
    fn execute_beq() {
        test_execute!(
            Instruction::BEQ { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BEQ { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BEQ { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 42, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BEQ { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 42, s3: 42}, pc: 52},
            results_in: {registers: {ra: 42, s3: 42}, pc: 8 },
        );
    }

    #[test]
    fn execute_beqz() {
        test_execute!(
            Instruction::BEQZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BEQZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BEQZ(Register::RA, -42),
            executed_on: {registers: {ra: 0}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_bne() {
        test_execute!(
            Instruction::BNE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BNE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 956 },
        );
        test_execute!(
            Instruction::BNE { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 42, s3: 41}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BNE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 42, s3: 41}, pc: 52},
            results_in: {registers: {ra: 42, s3: 41}, pc: 8 },
        );
    }

    #[test]
    fn execute_bnez() {
        test_execute!(
            Instruction::BNEZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BNEZ(Register::RA, -42),
            executed_on: {registers: {ra: 0}, pc: 1000},
            results_in: {registers: {ra: 0}, pc: 1004 },
        );
        test_execute!(
            Instruction::BNEZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_blt() {
        test_execute!(
            Instruction::BLT { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLT { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {s3: 1}, pc: 1000},
            results_in: {registers: {s3: 1}, pc: 956 },
        );
        test_execute!(
            Instruction::BLT { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 41, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BLT { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            results_in: {registers: {ra: 43, s3: 42}, pc: 56 },
        );
        test_execute!(
            Instruction::BLT { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: -42, s3: 43}, pc: 52},
            results_in: {registers: {ra: -42, s3: 43}, pc: 8 },
        );
    }

    #[test]
    fn execute_ble() {
        test_execute!(
            Instruction::BLE(Register::RA, Register::S3, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BLE(Register::RA, Register::S3, -42),
            executed_on: {registers: {s3: -1}, pc: 1000},
            results_in: {registers: {s3: -1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLE(Register::RA, Register::S3, -42),
            executed_on: {registers: {ra: 41, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BLE(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            results_in: {registers: {ra: 43, s3: 42}, pc: 56 },
        );
        test_execute!(
            Instruction::BLE(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: -42, s3: 43}, pc: 52},
            results_in: {registers: {ra: -42, s3: 43}, pc: 8 },
        );
    }

    #[test]
    fn execute_bltz() {
        test_execute!(
            Instruction::BLTZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLTZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLTZ(Register::RA, -42),
            executed_on: {registers: {ra: -1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_blez() {
        test_execute!(
            Instruction::BLEZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BLEZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLEZ(Register::RA, -42),
            executed_on: {registers: {ra: -1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_bge() {
        test_execute!(
            Instruction::BGE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BGE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 956 },
        );
        test_execute!(
            Instruction::BGE { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BGE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            results_in: {registers: {ra: 43, s3: 42}, pc: 8 },
        );
        test_execute!(
            Instruction::BGE { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: -41, s3: 43}, pc: 52},
            results_in: {registers: {ra: -41, s3: 43}, pc: 56 },
        );
    }

    #[test]
    fn execute_bgt() {
        test_execute!(
            Instruction::BGT(Register::RA, Register::T0, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGT(Register::RA, Register::T0, -42),
            executed_on: {registers: {ra: -1, t0: 3}, pc: 1000},
            results_in: {registers: {ra: -1, t0: 3}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGT(Register::RA, Register::T0,-42),
            executed_on: {registers: {ra: 1, t0: -1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_bgez() {
        test_execute!(
            Instruction::BGEZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BGEZ(Register::RA, -42),
            executed_on: {registers: {ra: -1}, pc: 1000},
            results_in: {registers: {ra: -1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGEZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_bgtz() {
        test_execute!(
            Instruction::BGTZ(Register::RA, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGTZ(Register::RA, -42),
            executed_on: {registers: {ra: -1}, pc: 1000},
            results_in: {registers: {ra: -1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGTZ(Register::RA, -42),
            executed_on: {registers: {ra: 1}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
    }

    #[test]
    fn execute_bltu() {
        test_execute!(
            Instruction::BLTU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLTU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {s3: 1}, pc: 1000},
            results_in: {registers: {s3: 1}, pc: 956 },
        );
        test_execute!(
            Instruction::BLTU { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 41, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BLTU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            results_in: {registers: {ra: 43, s3: 42}, pc: 56 },
        );
        test_execute!(
            Instruction::BLTU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: -42, s3: 43}, pc: 52},
            results_in: {registers: {ra: -42, s3: 43}, pc: 56 },
        );
    }

    #[test]
    fn execute_bgtu() {
        test_execute!(
            Instruction::BGTU(Register::RA, Register::S3, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGTU(Register::RA, Register::S3, -44),
            executed_on: {registers: {s3: 1}, pc: 1000},
            results_in: {registers: {s3: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BGTU(Register::RA, Register::S3, -42),
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BGTU(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 41, s3: 42}, pc: 52},
            results_in: {registers: {ra: 41, s3: 42}, pc: 56 },
        );
        test_execute!(
            Instruction::BGTU(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 42, s3: -43}, pc: 52},
            results_in: {registers: {ra: 42, s3: -43}, pc: 56 },
        );
    }

    #[test]
    fn execute_bgeu() {
        test_execute!(
            Instruction::BGEU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BGEU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 956 },
        );
        test_execute!(
            Instruction::BGEU { rs1: Register::RA, rs2: Register::S3, offset: -42 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BGEU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: 43, s3: 42}, pc: 52},
            results_in: {registers: {ra: 43, s3: 42}, pc: 8 },
        );
        test_execute!(
            Instruction::BGEU { rs1: Register::RA, rs2: Register::S3, offset: -44 },
            executed_on: {registers: {ra: -41, s3: 43}, pc: 52},
            results_in: {registers: {ra: -41, s3: 43}, pc: 8 },
        );
    }

    #[test]
    fn execute_bleu() {
        test_execute!(
            Instruction::BLEU(Register::RA, Register::S3, -44),
            executed_on: {registers: {}, pc: 1000},
            results_in: {registers: {}, pc: 956 },
        );
        test_execute!(
            Instruction::BLEU(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 1}, pc: 1000},
            results_in: {registers: {ra: 1}, pc: 1004 },
        );
        test_execute!(
            Instruction::BLEU(Register::RA, Register::S3, -42),
            executed_on: {registers: {ra: 41, s3: 42}, pc: 5},
            throws: Exception::MisalignedInstructionFetch
        );
        test_execute!(
            Instruction::BLEU(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 41, s3: 42}, pc: 52},
            results_in: {registers: {ra: 41, s3: 42}, pc: 8 },
        );
        test_execute!(
            Instruction::BLEU(Register::RA, Register::S3, -44),
            executed_on: {registers: {ra: 41, s3: -43}, pc: 52},
            results_in: {registers: {ra: 41, s3: -43}, pc: 8 },
        );
    }
}
