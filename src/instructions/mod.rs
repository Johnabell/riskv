#![allow(clippy::unusual_byte_groupings, clippy::upper_case_acronyms)]
mod csr;
mod csr_imm;
mod funct3;
mod funct6;
mod funct7;
mod immi;
mod immu;
mod impl_instruction_set;
mod pseudoinstructions;
mod rd;
mod rs1;
mod rs2;
mod shamt;
mod simmi;

use self::{
    csr::CSR, csr_imm::CsrImm, funct3::Funct3, funct6::Funct6, funct7::Funct7, immi::ImmI,
    immu::ImmU, rd::Rd, rs1::Rs1, rs2::Rs2, shamt::Shamt, simmi::SImmI,
};

use crate::registers::Register;

/// An representation of different instructions.
///
/// Would we like to work with the raw bytes of the instructions, or simply provide a mechanism to
/// convert to the raw bytes.
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Clone, Copy)]
pub(super) enum Instruction {
    /// # Load Upper Immediate
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

    /// # XOR Immediate
    ///
    /// Performs bitwise XOR on register rs1 and the sign-extended 12-bit immediate and place the
    /// result in rd.
    ///
    /// Note: "XORI rd, rs1, -1" performs a bitwise logical inversion of register rs1(assembler
    /// pseudo-instruction NOT rd, rs)
    ///
    /// `rd <- rs1 ^ sext(immediate)`
    XORI {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # OR Immediate
    ///
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit immediate and place the result in rd
    ///
    /// `rd <- rs1 | sext(immediate)`
    ORI {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # AND Immediate
    ///
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit immediate and place the
    /// result in rd
    ///
    /// `rd <- rs1 & sext(immediate)`
    ANDI {
        rd: Register,
        rs1: Register,
        imm: i16,
    },

    /// # Shift Logical Left Immediate
    ///
    /// Performs logical left shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits of the immediate
    /// In RV64, bit-25 is used to shamt[5].
    ///
    /// `rd <- rs1 << shamt`
    SLLI {
        rd: Register,
        rs1: Register,
        shamt: u8,
    },

    /// # Shift Logical Right Immediate
    ///
    /// Performs logical right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits of the immediate
    /// In RV64, bit-25 is used to shamt[5].
    ///
    /// `rd <- rs1 >>u shamt`
    SRLI {
        rd: Register,
        rs1: Register,
        shamt: u8,
    },

    /// # Shift Arithmetic Right Immediate
    ///
    /// Performs arithmetic right shift on the value in register rs1 by the shift amount held in
    /// the lower 5 bits of the immediate
    /// In RV64, bit-25 is used to shamt[5].
    ///
    /// `rd <- rs1 >>s shamt`
    SRAI {
        rd: Register,
        rs1: Register,
        shamt: u8,
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

    /// # Shift Left Logical
    ///
    /// Performs logical left shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit archeticture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 << rs2`
    SLL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Set Less Than
    ///
    /// Place the value 1 in register rd if register rs1 is less than register rs2 when both are
    /// treated as signed numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <s rs2`
    SLT {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Set Less Than Unsigned
    ///
    /// Place the value 1 in register rd if register rs1 is less than register rs2 when both are
    /// treated as unsigned numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <u rs2`
    SLTU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # XOR
    ///
    /// Performs bitwise XOR on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 ^ rs2`
    XOR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Shift Right Logical
    ///
    /// Performs logical right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit archeticture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 >> rs2`
    SRL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Shift Right Arithmetic
    ///
    /// Performs arithmetic right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit archeticture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 >>s rs2`
    SRA {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # OR
    ///
    /// Performs bitwise OR on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 | rs2`
    OR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # AND
    ///
    /// Performs bitwise AND on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 & rs2`
    AND {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// # Load Byte
    ///
    /// Loads a 8-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][7:0])`
    LB {
        rd: Register,
        rs1: Register,
        offset: i16,
    },

    /// # Load Half
    ///
    /// Loads a 16-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][15:0])`
    LH {
        rd: Register,
        rs1: Register,
        offset: i16,
    },

    /// # Load Word
    ///
    /// Loads a 32-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][31:0])`
    LW {
        rd: Register,
        rs1: Register,
        offset: i16,
    },

    /// # Load Byte Unsigned
    ///
    /// Loads a 8-bit value from memory and zero-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- M[rs1 + sext(offset)][7:0]`
    LBU {
        rd: Register,
        rs1: Register,
        offset: i16,
    },

    /// # Load Half Unsigned
    ///
    /// Loads a 16-bit value from memory and zero-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- M[rs1 + sext(offset)][15:0]`
    LHU {
        rd: Register,
        rs1: Register,
        offset: i16,
    },

    /// # Store Byte
    ///
    /// Store 8-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[7:0]`
    SB {
        rs1: Register,
        rs2: Register,
        offset: i16,
    },

    /// # Store Half
    ///
    /// Store 16-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[15:0]`
    SH {
        rs1: Register,
        rs2: Register,
        offset: i16,
    },

    /// # Store Word
    ///
    /// Store 32-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[31:0]`
    SW {
        rs1: Register,
        rs2: Register,
        offset: i16,
    },

    /// # Atomic CSR read write
    ///
    /// Atomically swaps values in the CSRs and integer registers.
    /// CSRRW reads the old value of the CSR, zero-extends the value to XLEN
    /// bits, then writes it to integer register rd.
    /// The initial value in rs1 is written to the CSR.
    /// If rd=x0, then the instruction shall not read the CSR and shall not
    /// cause any of the side effects that might occur on a CSR read.
    ///
    /// `t = CSRs[csr]; CSRs[csr] = rs1; rd = t`
    CSRRW {
        rd: Register,
        rs1: Register,
        csr: u16,
    },

    /// # Atomic CSR read write
    ///
    /// Reads the value of the CSR, zero-extends the value to XLEN bits, and
    /// writes it to integer register rd. The initial value in integer register
    /// rs1 is treated as a bit mask that specifies bit positions to be set in
    /// the CSR. Any bit that is high in rs1 will cause the corresponding bit
    /// to be set in the CSR, if that CSR bit is writable. Other bits in the
    /// CSR are unaffected (though CSRs might have side effects when written).
    ///
    /// `t = CSRs[csr]; CSRs[csr] = t | rs1; rd = t`
    CSRRS {
        rd: Register,
        rs1: Register,
        csr: u16,
    },

    /// # Atomic CSR read write
    ///
    /// Reads the value of the CSR, zero-extends the value to XLEN bits, and
    /// writes it to integer register rd. The initial value in integer register
    /// rs1 is treated as a bit mask that specifies bit positions to be cleared
    /// in the CSR. Any bit that is high in rs1 will cause the corresponding
    /// bit to be cleared in the CSR, if that CSR bit is writable. Other bits
    /// in the CSR are unaffected.
    ///
    /// `t = CSRs[csr]; CSRs[csr] = t & ∼rs1; rd = t`
    CSRRC {
        rd: Register,
        rs1: Register,
        csr: u16,
    },

    /// # Atomic CSR read write immediate
    ///
    /// Update the CSR using an XLEN-bit value obtained by zero-extending a
    /// 5-bit unsigned immediate (uimm[4:0]) field encoded in the rs1 field.
    ///
    /// `rd = CSRs[csr]; CSRs[csr] = zext(imm)`
    CSRRWI { rd: Register, csr: u16, imm: u8 },

    /// # Atomic CSR read set immediate
    ///
    /// Set CSR bit using an XLEN-bit value obtained by zero-extending a 5-bit
    /// unsigned immediate (uimm[4:0]) field encoded in the rs1 field.
    ///
    /// `t = CSRs[csr]; CSRs[csr] = t | zext(imm); rd = t`
    CSRRSI { rd: Register, csr: u16, imm: u8 },
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
                0b_001 => Instruction::SLLI {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    shamt: *Shamt::from(value),
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
                0b_100 => Instruction::XORI {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    imm: *ImmI::from(value),
                },
                0b_101 => match *Funct6::from(value) {
                    0b_000000 => Instruction::SRLI {
                        rd: *Rd::from(value),
                        rs1: *Rs1::from(value),
                        shamt: *Shamt::from(value),
                    },
                    0b_010000 => Instruction::SRAI {
                        rd: *Rd::from(value),
                        rs1: *Rs1::from(value),
                        shamt: *Shamt::from(value),
                    },
                    _ => unimplemented!(
                        "The given instruction is not yet implemented {:#034b}",
                        value.to_le()
                    ),
                },
                0b_110 => Instruction::ORI {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    imm: *ImmI::from(value),
                },
                0b_111 => Instruction::ANDI {
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
                (0b_001, 0b_0000000) => Instruction::SLL {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_010, 0b_0000000) => Instruction::SLT {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_011, 0b_0000000) => Instruction::SLTU {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_100, 0b_0000000) => Instruction::XOR {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_101, 0b_0000000) => Instruction::SRL {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_101, 0b_0100000) => Instruction::SRA {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_110, 0b_0100000) => Instruction::OR {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                (0b_111, 0b_0100000) => Instruction::AND {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                },
                _ => unimplemented!(
                    "The given instruction is not yet implemented {:#034b}",
                    value.to_le()
                ),
            },
            0b_0000011 => match *Funct3::from(value) {
                0b_000 => Instruction::LB {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    offset: *ImmI::from(value),
                },
                0b_001 => Instruction::LH {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    offset: *ImmI::from(value),
                },
                0b_010 => Instruction::LW {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    offset: *ImmI::from(value),
                },
                0b_100 => Instruction::LBU {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    offset: *ImmI::from(value),
                },
                0b_101 => Instruction::LHU {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    offset: *ImmI::from(value),
                },
                _ => unimplemented!(
                    "The given instruction is not yet implemented {:#034b}",
                    value.to_le()
                ),
            },
            0b_0100011 => match *Funct3::from(value) {
                0b_000 => Instruction::SB {
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                    offset: *SImmI::from(value),
                },
                0b_001 => Instruction::SH {
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                    offset: *SImmI::from(value),
                },
                0b_010 => Instruction::SW {
                    rs1: *Rs1::from(value),
                    rs2: *Rs2::from(value),
                    offset: *SImmI::from(value),
                },
                _ => unimplemented!(
                    "The given instruction is not yet implemented {:#034b}",
                    value.to_le()
                ),
            },
            0b_1110011 => match *Funct3::from(value) {
                0b_001 => Instruction::CSRRW {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    csr: *CSR::from(value),
                },
                0b_010 => Instruction::CSRRS {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    csr: *CSR::from(value),
                },
                0b_011 => Instruction::CSRRC {
                    rd: *Rd::from(value),
                    rs1: *Rs1::from(value),
                    csr: *CSR::from(value),
                },
                0b_101 => Instruction::CSRRWI {
                    rd: *Rd::from(value),
                    imm: *CsrImm::from(value),
                    csr: *Csr::from(value),
                },
                0b_110 => Instruction::CSRRSI {
                    rd: *Rd::from(value),
                    imm: *CsrImm::from(value),
                    csr: *Csr::from(value),
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
    fn lui_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_101_11000_0110111)),
            Instruction::LUI {
                rd: Register::S8,
                imm: 0x48A65
            }
        );
    }

    #[test]
    fn auipc_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_111_11100_0010111)),
            Instruction::AUIPC {
                rd: Register::T3,
                imm: 0x48A67
            }
        );
    }

    #[test]
    fn addi_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_00000_00000_000_00001_0010011)),
            Instruction::ADDI {
                rd: Register::RA,
                rs1: Register::ZERO,
                imm: 32
            }
        );
    }

    #[test]
    fn slti_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_00000_00100_010_00011_0010011)),
            Instruction::SLTI {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 32
            }
        );
    }

    #[test]
    fn sltiu_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000011_00000_00100_011_00011_0010011)),
            Instruction::SLTIU {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 96
            }
        );
    }

    #[test]
    fn xori_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_1111111_11000_01100_100_01011_0010011)),
            Instruction::XORI {
                rd: Register::A1,
                rs1: Register::A2,
                imm: -8
            }
        );
    }

    #[test]
    fn ori_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_1111111_01000_01101_110_10011_0010011)),
            Instruction::ORI {
                rd: Register::S3,
                rs1: Register::A3,
                imm: -24
            }
        );
    }

    #[test]
    fn andi_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000010_01000_11100_111_01111_0010011)),
            Instruction::ANDI {
                rd: Register::A5,
                rs1: Register::T3,
                imm: 72
            }
        );
    }

    #[test]
    fn slli_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_01010_10010_001_01101_0010011)),
            Instruction::SLLI {
                rd: Register::A3,
                rs1: Register::S2,
                shamt: 10
            }
        );
    }

    #[test]
    fn srli_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_10011_101_01110_0010011)),
            Instruction::SRLI {
                rd: Register::A4,
                rs1: Register::S3,
                shamt: 42
            }
        );
    }

    #[test]
    fn srai_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11010_10100_101_10000_0010011)),
            Instruction::SRAI {
                rd: Register::A6,
                rs1: Register::S4,
                shamt: 26
            }
        );
    }

    #[test]
    fn add_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_01101_01011_000_00010_0110011)),
            Instruction::ADD {
                rd: Register::SP,
                rs1: Register::A1,
                rs2: Register::A3,
            }
        );
    }

    #[test]
    fn sub_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11101_11011_000_00010_0110011)),
            Instruction::SUB {
                rd: Register::SP,
                rs1: Register::S11,
                rs2: Register::T4,
            }
        );
    }

    #[test]
    fn sll_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_10110_10101_001_10100_0110011)),
            Instruction::SLL {
                rd: Register::S4,
                rs1: Register::S5,
                rs2: Register::S6,
            }
        );
    }

    #[test]
    fn stl_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_11100_10011_010_00110_0110011)),
            Instruction::SLT {
                rd: Register::T1,
                rs1: Register::S3,
                rs2: Register::T3,
            }
        );
    }

    #[test]
    fn stlu_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_11000_10001_011_01110_0110011)),
            Instruction::SLTU {
                rd: Register::A4,
                rs1: Register::A7,
                rs2: Register::S8,
            }
        );
    }

    #[test]
    fn xor_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_00111_01010_100_00101_0110011)),
            Instruction::XOR {
                rd: Register::T0,
                rs1: Register::A0,
                rs2: Register::T2,
            }
        );
    }

    #[test]
    fn srl_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_11001_11000_101_10111_0110011)),
            Instruction::SRL {
                rd: Register::S7,
                rs1: Register::S8,
                rs2: Register::S9,
            }
        );
    }

    #[test]
    fn sra_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11100_11011_101_11010_0110011)),
            Instruction::SRA {
                rd: Register::S10,
                rs1: Register::S11,
                rs2: Register::T3,
            }
        );
    }

    #[test]
    fn or_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11100_01001_110_01000_0110011)),
            Instruction::OR {
                rd: Register::S0,
                rs1: Register::S1,
                rs2: Register::T3,
            }
        );
    }

    #[test]
    fn and_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100000_11111_11110_111_11101_0110011)),
            Instruction::AND {
                rd: Register::T4,
                rs1: Register::T5,
                rs2: Register::T6,
            }
        );
    }

    #[test]
    fn lb_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11001_01100_000_11100_0000011)),
            Instruction::LB {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 57,
            }
        );
    }

    #[test]
    fn lh_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11010_01100_001_11100_0000011)),
            Instruction::LH {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 58,
            }
        );
    }

    #[test]
    fn lw_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11000_01100_010_11100_0000011)),
            Instruction::LW {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 56,
            }
        );
    }

    #[test]
    fn lbu_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11011_01100_100_11100_0000011)),
            Instruction::LBU {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 59,
            }
        );
    }

    #[test]
    fn lhu_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11100_01100_101_11100_0000011)),
            Instruction::LHU {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 60,
            }
        );
    }

    #[test]
    fn sb_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11101_01101_000_11101_0100011)),
            Instruction::SB {
                rs1: Register::A3,
                rs2: Register::T4,
                offset: 61,
            }
        );
    }

    #[test]
    fn sh_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11101_01101_001_11110_0100011)),
            Instruction::SH {
                rs1: Register::A3,
                rs2: Register::T4,
                offset: 62,
            }
        );
    }

    #[test]
    fn sw_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11111_01101_010_11111_0100011)),
            Instruction::SW {
                rs1: Register::A3,
                rs2: Register::T6,
                offset: 63,
            }
        );
    }

    #[test]
    fn csrrw_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0001001_11111_01111_001_11011_1110011)),
            Instruction::CSRRW {
                rd: Register::S11,
                rs1: Register::A5,
                csr: 319,
            }
        );
    }

    #[test]
    fn csrrs_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0101001_11111_01011_010_10011_1110011)),
            Instruction::CSRRS {
                rd: Register::S3,
                rs1: Register::A1,
                csr: 1343,
            }
        );
    }

    #[test]
    fn csrrc_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_1101001_11111_01001_011_10111_1110011)),
            Instruction::CSRRC {
                rd: Register::S7,
                rs1: Register::S1,
                csr: 3391,
            }
        );
    }

    #[test]
    fn csrrwi_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_11011_01001_101_10101_1110011)),
            Instruction::CSRRWI {
                rd: Register::S5,
                imm: 9,
                csr: 59,
            }
        );
    }

    #[test]
    fn csrrsi_from_i32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_11011_110_10100_1110011)),
            Instruction::CSRRSI {
                rd: Register::S4,
                imm: 27,
                csr: 42,
            }
        );
    }
}
