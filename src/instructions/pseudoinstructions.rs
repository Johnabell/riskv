//! RISC-V Pseudoinstruction
use crate::{integer::i12, registers::Register};

use super::{immu::ImmU, Instruction};

/// An iterator of up to 3 instructions.
///
/// All pseudoinstructions desugar to a number of instructions.
///
/// This is encapsulated by this iterator.
pub(crate) enum PseudoinstructionMappingIter {
    /// The iterator which will yield three instructions.
    Three(Instruction, Instruction, Instruction),
    /// The iterator which will yield two instructions.
    Two(Instruction, Instruction),
    /// The iterator which will yield one instruction.
    One(Instruction),
    /// The iterator which will yield no instructions.
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
    /// Note: This pseudoinstruction desugars to a load upper Immediate
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
                    imm: i12::sign_extend(imm),
                },
            )
        }
    }

    /// # Bitwise NOT
    ///
    /// Performs a bitwise logical inversion of register rs and places the result in rd.
    ///
    /// Note: This pseudoinstruction desugars to `XORI rd, rs, -1`.
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
    /// Note: This pseudoinstruction desugars to `SUB rd, x0, rs`.
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
    /// Note: This pseudoinstruction desugars to `ADDI rd, rs, 0`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn MOV(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::ADDI {
            rd,
            rs1: rs,
            imm: 0,
        })
    }

    /// # Set Equal Zero
    ///
    /// Sets the destination register to 1 if `rs` is zero, otherwises set the destination register
    /// to 0.
    ///
    /// Note: This pseudoinstruction desugars to `SLTUI rd, rs, 1`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn SEQZ(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::SLTIU {
            rd,
            rs1: rs,
            imm: 1,
        })
    }

    /// # Set Not Equal Zero
    ///
    /// Sets the destination register to 1 if `rs` is not equal to zero, otherwises set the destination register
    /// to 0.
    ///
    /// Note: This pseudoinstruction desugars to `SLTU rd, x0, rs`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn SNEZ(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::SLTU {
            rd,
            rs1: Register::ZERO,
            rs2: rs,
        })
    }

    /// # Set Less Than Zero
    ///
    /// Sets the destination register to 1 if `rs` is less than zero, otherwises set the destination register
    /// to 0.
    ///
    /// Note: This pseudoinstruction desugars to `SLT rd, rs, x0`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn SLTZ(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::SLT {
            rd,
            rs1: rs,
            rs2: Register::ZERO,
        })
    }

    /// # Set Greater Than Zero
    ///
    /// Sets the destination register to 1 if `rs` is greater than zero, otherwises set the destination register
    /// to 0.
    ///
    /// Note: This pseudoinstruction desugars to `SLT rd, x0, rs`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) fn SGLZ(rd: Register, rs: Register) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::SLT {
            rd,
            rs1: Register::ZERO,
            rs2: rs,
        })
    }

    /// # NOP
    ///
    /// This instruction does nothing.
    ///
    /// Note: This pseudoinstruction desugars to `ADDI x0, x0, 0`.
    /// See [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions).
    #[allow(non_snake_case)]
    pub(crate) const NOP: PseudoinstructionMappingIter =
        PseudoinstructionMappingIter::One(Instruction::ADDI {
            rd: Register::ZERO,
            rs1: Register::ZERO,
            imm: 0,
        });

    /// # Read CSR
    ///
    /// Read CSR
    ///
    /// Note: This pseudoinstruction desugars to `CSRRW rd, csr, x0`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRR(rd: Register, csr: u16) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRW {
            rd,
            rs1: Register::ZERO,
            csr,
        })
    }

    /// # Write CSR
    ///
    /// Write CSR, no read side effects should be caused by this instruction.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRW x0, csr, rs`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRW(rs1: Register, csr: u16) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRW {
            rd: Register::ZERO,
            rs1,
            csr,
        })
    }

    /// # Set bits in CSR
    ///
    /// Sets the bits in CSR, no read side effects should be caused by this
    /// instruction.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRS x0, csr, rs`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRS(rs1: Register, csr: u16) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRS {
            rd: Register::ZERO,
            rs1,
            csr,
        })
    }

    /// # Clear bits in CSR
    ///
    /// Clear bits in CSR, no read side effects should be caused by this
    /// instruction.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRC x0, csr, rs`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRC(rs1: Register, csr: u16) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRC {
            rd: Register::ZERO,
            rs1,
            csr,
        })
    }

    /// # Write CSR, immediate
    ///
    /// Writes into CSR, using the immediate value. This instruction should
    /// not cause any read side effects.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRWI x0, csr, imm`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRWI(csr: u16, imm: u8) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRWI {
            rd: Register::ZERO,
            imm,
            csr,
        })
    }

    /// # Set bits in CSR, immediate
    ///
    /// Sets bits in CSR using the immediate value. This instruction should
    /// not cause any read side effects.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRSI x0, csr, imm`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRSI(csr: u16, imm: u8) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRSI {
            rd: Register::ZERO,
            imm,
            csr,
        })
    }

    /// # Clear bits in CSR, immediate
    ///
    /// Clears bits in CSR using the immediate value. This instruction should
    /// not cause any read side effects.
    ///
    /// Note: This pseudoinstruction desugars to `CSRRCI x0, csr, imm`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#pseudoinstructions-for-accessing-control-and-status-registers)
    #[allow(non_snake_case)]
    pub(crate) fn CSRCI(csr: u16, imm: u8) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::CSRRCI {
            rd: Register::ZERO,
            imm,
            csr,
        })
    }

    /// # Jump and link
    ///
    /// Jump and link using the default return address register.
    ///
    /// Note: This pseudoinstruction desugars to `JAL x1, offset`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions)
    #[allow(non_snake_case)]
    pub(crate) fn JAL(offset: i32) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::JAL {
            rd: Register::RA,
            offset,
        })
    }

    /// # Unconditional Jump
    ///
    /// Jump to the relative offset without linking the return address
    ///
    /// Note: This pseudoinstruction desugars to `JAL x0, offset`
    /// See
    /// [ref](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md#-a-listing-of-standard-risc-v-pseudoinstructions)
    #[allow(non_snake_case)]
    pub(crate) fn J(offset: i32) -> PseudoinstructionMappingIter {
        PseudoinstructionMappingIter::One(Instruction::JAL {
            rd: Register::ZERO,
            offset,
        })
    }
}

/// If the `i12` value is negative returns `1` otherwise returns `0`.
fn with_signed_i12_adjustment(value: i32) -> i32 {
    if i12::is_positive(value) {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn iter_test() {
        let pseudoinstruction = PseudoinstructionMappingIter::Three(
            Instruction::ADD {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A2,
            },
            Instruction::SUB {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A1,
            },
            Instruction::OR {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A1,
            },
        );
        assert_eq!(
            pseudoinstruction.collect::<Vec<_>>(),
            vec![
                Instruction::ADD {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A2,
                },
                Instruction::SUB {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A1,
                },
                Instruction::OR {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A1,
                },
            ]
        );
    }

    #[test]
    fn rev_iter_test() {
        let pseudoinstruction = PseudoinstructionMappingIter::Three(
            Instruction::ADD {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A2,
            },
            Instruction::SUB {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A1,
            },
            Instruction::OR {
                rd: Register::SP,
                rs1: Register::A0,
                rs2: Register::A1,
            },
        );
        assert_eq!(
            pseudoinstruction.rev().collect::<Vec<_>>(),
            vec![
                Instruction::OR {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A1,
                },
                Instruction::SUB {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A1,
                },
                Instruction::ADD {
                    rd: Register::SP,
                    rs1: Register::A0,
                    rs2: Register::A2,
                },
            ]
        );
    }
}
