#![allow(clippy::unusual_byte_groupings, clippy::upper_case_acronyms)]
mod funct3;
mod funct7;
mod immi;
mod immu;
mod pseudoinstructions;
mod rd;
mod rs1;
mod rs2;

use self::{funct3::Funct3, funct7::Funct7, immi::ImmI, immu::ImmU, rd::Rd, rs1::Rs1, rs2::Rs2};

use crate::registers::Register;
/// An representation of different instructions.
///
/// Would we like to work with the raw bytes of the instructions, or simply provide a mechanism to
/// convert to the raw bytes.
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Clone, Copy)]
pub(super) enum Instruction {
    /// Load Upper Immediate
    ///
    /// Build 32-bit constants and uses the U-type format. LUI places the U-immediate value in the
    /// top 20 bits of the destination register rd, filling in the lowest 12 bits with zeros.
    ///
    /// `rd <- sext(immediate[31:12] << 12)`
    LUI { rd: Register, imm: i32 },

    /// # Add upper immediate to programme counter
    ///
    /// Build pc-relative addresses and uses the U-type format. AUIPC forms a 32-bit offset from
    /// the 20-bit U-immediate, filling in the lowest 12 bits with zeros, adds this offset to the
    /// pc, then places the result in register rd.
    ///
    /// `rd <- pc + sext(immediate[31:12] << 12)`
    AUIPC { rd: Register, imm: i32 },

    /// # Add Immediate
    ///
    /// Adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and
    /// the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement
    /// the MV rd, rs1 assembler pseudo-instruction.
    ///
    /// `rd <- rs1 + rs2`
    ADDI {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # Set Less Than Immediate
    ///
    /// Place the value 1 in register rd if register rs1 is less than the signextended immediate
    /// when both are treated as signed numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <s sext(immediate)`
    SLTI {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # Set Less Than Immediate Unsigned
    ///
    /// Place the value 1 in register rd if register rs1 is less than the immediate when both are
    /// treated as unsigned numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <u sext(immediate)`
    SLTIU {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # Add
    ///
    /// Adds the registers rs1 and rs2 and stores the result in rd.
    /// Arithmetic overflow is ignored and the result is simply the low XLEN bits of the
    /// result.
    ///
    /// `rd <- rs1 + rs2`
    ADD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Subract
    ///
    /// Arithmetic overflow is ignored and the result is simply the low XLEN bits of the
    /// result.
    /// placing the output in `rd`
    ///
    /// `rd <- rs1 - rs2`
    SUB {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        match Instruction::op_code(value) {
            0b_0110111 => Instruction::LUI {
                rd: *Rd::from(value),
                imm: *ImmU::from(value),
            },
            0b_0010111 => Instruction::AUIPC {
                rd: *Rd::from(value),
                imm: *ImmU::from(value),
            },
            0b_0010011 => match *Funct3::from(value) {
                0b_000 => Instruction::ADDI {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    imm: *ImmI::from(value),
                },
                0b_010 => Instruction::SLTI {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    imm: *ImmI::from(value),
                },
                0b_011 => Instruction::SLTIU {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    imm: *ImmI::from(value),
                },
                _ => unimplemented!(
                    "The given instruction is not yet implemented {:#034b}",
                    value.to_le()
                ),
            },
            0b_0110011 => match (*Funct3::from(value), *Funct7::from(value)) {
                (0b_000, 0b_0000000) => Instruction::ADD {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_000, 0b_0100000) => Instruction::SUB {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                _ => unimplemented!(
                    "The given instruction is not yet implemented {:#034b}",
                    value.to_le()
                ),
            },
            _ => unimplemented!(
                "The given instruction is not yet implemented {:#034b}",
                value.to_le()
            ),
        }
    }
}

impl Instruction {
    fn op_code(value: u32) -> u8 {
        (value & OPP_MASK) as u8
    }
}

const OPP_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_00000_1111111);

#[cfg(test)]
mod test {
    use crate::{instructions::Instruction, registers::Register};

    #[test]
    fn from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_101_11000_0110111)),
            Instruction::LUI {
                rd: Register::S8,
                imm: 0x48A65
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_111_11100_0010111)),
            Instruction::AUIPC {
                rd: Register::T3,
                imm: 0x48A67
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_00000_00000_000_00001_0010011)),
            Instruction::ADDI {
                rd: Register::RA,
                rs1: Register::ZERO,
                imm: 32
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_00000_00100_010_00011_0010011)),
            Instruction::SLTI {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 32
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000011_00000_00100_011_00011_0010011)),
            Instruction::SLTIU {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 96
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_01101_01011_000_00010_0110011)),
            Instruction::ADD {
                rd: Register::SP,
                rs1: Register::A1,
                rs2: Register::A3,
            }
        );
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11101_11011_000_00010_0110011)),
            Instruction::SUB {
                rd: Register::SP,
                rs1: Register::S11,
                rs2: Register::T4,
            }
        );
    }
}
