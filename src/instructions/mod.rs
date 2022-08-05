use super::Register;
use num::Num;
/// An representation of different instructions.
///
/// Would we like to work with the raw bytes of the instructions, or simply provide a mechanism to
/// convert to the raw bytes.
pub(super) enum Instruction<T>
where
    T: Num,
{
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
    ADDI { rd: Register, rs1: Register, imm: T },

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
    LI { rd: Register, imm: T },
}
