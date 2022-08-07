mod funct3;
mod funct7;
mod immi;
mod rd;
mod rs1;
mod rs2;

use self::{funct3::Funct3, funct7::Funct7, immi::ImmI, rd::Rd, rs1::Rs1, rs2::Rs2};

use super::Register;
/// An representation of different instructions.
///
/// Would we like to work with the raw bytes of the instructions, or simply provide a mechanism to
/// convert to the raw bytes.
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(super) enum Instruction {
    /// Integer ADD instruction to add the values in `rs1` and `rs2` and
    /// place the output in `rd`
    /// rd <- rs1 + rs2
    ADD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// Integer ADD immediate instruction to take the value in `rs1` and add `imm`
    /// placing the output in `rd`
    /// rd <- rs1 + rs2
    ADDI {
        rd: Register,
        rs1: Register,
        imm: u16,
    },

    /// Integer SUB instruction to take the value in `rs1` and subtract `rs2`  
    /// placing the output in `rd`
    /// rd <- rs1 - rs2
    SUB {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    /// Load Immediate
    ///
    /// Note: in RISK-V this is a Psudo Instruction that desugars to a load upper Immediate
    /// and a add immediate for the lower bits.
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#load-immediate).
    ///
    /// For now we are treating this as a single instruction
    LI { rd: Register, imm: u32 },
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        match Instruction::op_code(value) {
            0b_0010011 => match *Funct3::from(value) {
                0b_000 => Instruction::ADDI {
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
    use crate::{instructions::Instruction, Register};

    #[test]
    fn from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_00000_00000_000_00001_0010011)),
            Instruction::ADDI {
                rd: Register::RA,
                rs1: Register::ZERO,
                imm: 32
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
