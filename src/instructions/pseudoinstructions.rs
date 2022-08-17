use crate::{integer::i12, registers::Register};

use super::{immu::ImmU, Instruction};

pub(crate) enum PseudoinstructionMappingIter {
    Three(Instruction, Instruction, Instruction),
    Two(Instruction, Instruction),
    One(Instruction),
    Zero,
}

impl Iterator for PseudoinstructionMappingIter {
    type Item = Instruction;
    fn next(&mut self) -> Option<Instruction> {
        match *self {
            PseudoinstructionMappingIter::Three(a, b, c) => {
                *self = PseudoinstructionMappingIter::Two(b, c);
                Some(a)
            }
            PseudoinstructionMappingIter::Two(b, c) => {
                *self = PseudoinstructionMappingIter::One(c);
                Some(b)
            }
            PseudoinstructionMappingIter::One(c) => {
                *self = PseudoinstructionMappingIter::Zero;
                Some(c)
            }
            PseudoinstructionMappingIter::Zero => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = match self {
            PseudoinstructionMappingIter::Three(..) => 3,
            PseudoinstructionMappingIter::Two(..) => 2,
            PseudoinstructionMappingIter::One(_) => 1,
            PseudoinstructionMappingIter::Zero => 0,
        };
        (size, Some(size))
    }
}

impl DoubleEndedIterator for PseudoinstructionMappingIter {
    fn next_back(&mut self) -> Option<Instruction> {
        match *self {
            PseudoinstructionMappingIter::Three(a, b, c) => {
                *self = PseudoinstructionMappingIter::Two(a, b);
                Some(c)
            }
            PseudoinstructionMappingIter::Two(b, c) => {
                *self = PseudoinstructionMappingIter::One(b);
                Some(c)
            }
            PseudoinstructionMappingIter::One(c) => {
                *self = PseudoinstructionMappingIter::Zero;
                Some(c)
            }
            PseudoinstructionMappingIter::Zero => None,
        }
    }
}

impl Instruction {
    /// # Load Immediate
    ///
    /// Note: This pseudo Instruction desugars to a load upper Immediate
    /// and a add immediate for the lower bits.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#load-immediate).
    #[allow(non_snake_case)]
    pub(crate) fn LI(rd: Register, imm: i32) -> PseudoinstructionMappingIter {
        if imm >= i12::MIN as i32 && imm <= i12::MAX as i32 {
            PseudoinstructionMappingIter::One(Instruction::ADDI {
                rd,
                rs1: Register::ZERO,
                imm: imm as i16,
            })
        } else {
            PseudoinstructionMappingIter::Two(
                Instruction::LUI {
                    rd,
                    imm: (imm >> ImmU::RSHIFT) + with_signed_i12_adjustment(imm),
                },
                Instruction::ADDI {
                    rd,
                    rs1: rd,
                    imm: sign_extend_i12(imm),
                },
            )
        }
    }

    /// # Bitwise NOT
    ///
    /// Performs a bitwise logical inversion of register rs and places the result in rd.
    ///
    /// Note: This pseudo Instruction desugars to `XORI rd, rs, -1`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn NOT(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::XORI {
            rd,
            rs1: rs,
            imm: -1,
        })
    }

    /// # Negative
    ///
    /// Two compliment negation.
    ///
    /// Note: This pseudo Instruction desugars to `SUB rd, x0, rs`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn NEG(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::SUB {
            rd,
            rs1: Register::ZERO,
            rs2: rs,
        })
    }

    /// # Move
    ///
    /// Move the value in rs to rd.
    ///
    /// Note: This pseudo Instruction desugars to `ADDI rd, rs, 0`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn MOV(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::ADDI {
            rd,
            rs1: rs,
            imm: 0,
        })
    }

    /// # NOP
    ///
    /// This instruction does nothing.
    ///
    /// Note: This pseudo Instruction desugars to `ADDI x0, x0, 0`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) const NOP: PseudoinstructionMappingIter =
        PseudoinstructionMappingIter::One(Instruction::ADDI {
            rd: Register::ZERO,
            rs1: Register::ZERO,
            imm: 0,
        });
}

fn sign_extend_i12(value: i32) -> i16 {
    if is_positive_i12(value) {
        (value & 0x7FF) as i16
    } else {
        (value & 0xFFF | 0xF000) as i16
    }
}

fn is_positive_i12(value: i32) -> bool {
    value & 0b_1000_0000_0000 == 0
}

fn with_signed_i12_adjustment(value: i32) -> i32 {
    if is_positive_i12(value) {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sign_extend_i12_test() {
        let neg_1 = 0b_1111_1111_1111 as i32;
        let max_i12 = 0b_0111_1111_1111 as i32;
        let min_i12 = 0b_1000_0000_0000 as i32;
        assert_eq!(sign_extend_i12(neg_1), -1);
        assert_eq!(sign_extend_i12(max_i12), i12::MAX);
        assert_eq!(sign_extend_i12(min_i12), i12::MIN);
    }

    #[test]
    fn is_positive_i12_test() {
        let neg_1 = 0b_1111_1111_1111 as i32;
        let max_i12 = 0b_0111_1111_1111 as i32;
        assert!(!is_positive_i12(neg_1));
        assert!(is_positive_i12(max_i12));
    }
}
