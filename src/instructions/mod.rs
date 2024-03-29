//! Types representing RISC-V instructions.
//!
//! [Instruction] is primarily the definition of the RV32I instructions and
//! [crate::instruction_set::InstructionSet] has been implemented for it.
//!
//! This module also contains a number of helpers for exacting the different part of
//! an encoded instruction.
#![allow(clippy::unusual_byte_groupings, clippy::upper_case_acronyms)]
mod bimm;
mod csr;
mod csr_imm;
mod funct3;
mod funct6;
mod funct7;
mod immi;
mod immu;
mod impl_instruction_set;
mod jimm;
mod pseudoinstructions;
mod rd;
mod rs1;
mod rs2;
mod shamt;
mod simmi;
mod types;

use self::{
    bimm::BImm, csr::Csr, csr_imm::CsrImm, funct3::Funct3, funct6::Funct6, funct7::Funct7,
    immi::ImmI, immu::ImmU, jimm::JImm, rd::Rd, rs1::Rs1, rs2::Rs2, shamt::Shamt, simmi::SImmI,
};

use crate::{instruction_set::Exception, registers::Register};

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
    LUI {
        /// The destination register
        rd: Register,
        /// The 20-bit immediate value sign extended to an [i32].
        imm: i32,
    },

    /// # Add upper immediate to programme counter
    ///
    /// Build pc-relative addresses and uses the U-type format. AUIPC forms a 32-bit offset from
    /// the 20-bit U-immediate, filling in the lowest 12 bits with zeros, adds this offset to the
    /// pc, then places the result in register rd.
    ///
    /// `rd <- pc + sext(immediate[31:12] << 12)`
    AUIPC {
        /// The destination register
        rd: Register,
        /// The 20-bit immediate value sign extended to an [i32].
        imm: i32,
    },

    /// # Add Immediate
    ///
    /// Adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and
    /// the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement
    /// the MV rd, rs1 assembler pseudo-instruction.
    ///
    /// `rd <- rs1 + sext(immediate)`
    ADDI {
        /// The destination register.
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
        imm: i16,
    },

    /// # Set Less Than Immediate
    ///
    /// Place the value 1 in register rd if register rs1 is less than the signextended immediate
    /// when both are treated as signed numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <s sext(immediate)`
    SLTI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
        imm: i16,
    },

    /// # Set Less Than Immediate Unsigned
    ///
    /// Place the value 1 in register rd if register rs1 is less than the immediate when both are
    /// treated as unsigned numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <u sext(immediate)`
    SLTIU {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
        imm: i16,
    },

    /// # OR Immediate
    ///
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit immediate and place the result in rd
    ///
    /// `rd <- rs1 | sext(immediate)`
    ORI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
        imm: i16,
    },

    /// # AND Immediate
    ///
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit immediate and place the
    /// result in rd
    ///
    /// `rd <- rs1 & sext(immediate)`
    ANDI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit immediate value sign extended to an [i16].
        imm: i16,
    },

    /// # Shift Logical Left Immediate
    ///
    /// Performs logical left shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits of the immediate
    /// In RV64, bit-25 is used for `shamt[5]`.
    ///
    /// `rd <- rs1 << shamt`
    SLLI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 5-bit (for 32-bit architecture) shift amount zero extened to a [u8].
        ///
        /// Note: on 64-bit this will be a 6-bit value.
        shamt: u8,
    },

    /// # Shift Logical Right Immediate
    ///
    /// Performs logical right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits of the immediate
    /// In RV64, bit-25 is used for `shamt[5]`.
    ///
    /// `rd <- rs1 >>u shamt`
    SRLI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 5-bit (for 32-bit architecture) shift amount zero extened to a [u8].
        ///
        /// Note: on 64-bit this will be a 6-bit value.
        shamt: u8,
    },

    /// # Shift Arithmetic Right Immediate
    ///
    /// Performs arithmetic right shift on the value in register rs1 by the shift amount held in
    /// the lower 5 bits of the immediate
    /// In RV64, bit-25 is used for `shamt[5]`.
    ///
    /// `rd <- rs1 >>s shamt`
    SRAI {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 5-bit (for 32-bit architecture) shift amount zero extened to a [u8].
        ///
        /// Note: on 64-bit this will be a 6-bit value.
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Shift Left Logical
    ///
    /// Performs logical left shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit architecture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 << rs2`
    SLL {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Set Less Than
    ///
    /// Place the value 1 in register rd if register rs1 is less than register rs2 when both are
    /// treated as signed numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <s rs2`
    SLT {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Set Less Than Unsigned
    ///
    /// Place the value 1 in register rd if register rs1 is less than register rs2 when both are
    /// treated as unsigned numbers, else 0 is written to rd.
    ///
    /// `rd <- rs1 <u rs2`
    SLTU {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # XOR
    ///
    /// Performs bitwise XOR on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 ^ rs2`
    XOR {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Shift Right Logical
    ///
    /// Performs logical right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit architecture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 >> rs2`
    SRL {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Shift Right Arithmetic
    ///
    /// Performs arithmetic right shift on the value in register rs1 by the shift amount held in the
    /// lower 5 bits (for 32-bit architecture) or 6 bits (64-bit archetecture) of register rs2.
    ///
    /// `rd = rs1 >>s rs2`
    SRA {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # OR
    ///
    /// Performs bitwise OR on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 | rs2`
    OR {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # AND
    ///
    /// Performs bitwise AND on registers rs1 and rs2 and place the result in rd.
    ///
    /// `rd = rs1 & rs2`
    AND {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
    },

    /// # Load Byte
    ///
    /// Loads a 8-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][7:0])`
    LB {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Load Half
    ///
    /// Loads a 16-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][15:0])`
    LH {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Load Word
    ///
    /// Loads a 32-bit value from memory and sign-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- sext(M[rs1 + sext(offset)][31:0])`
    LW {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Load Byte Unsigned
    ///
    /// Loads a 8-bit value from memory and zero-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- M[rs1 + sext(offset)][7:0]`
    LBU {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Load Half Unsigned
    ///
    /// Loads a 16-bit value from memory and zero-extends this to XLEN bits before storing it in
    /// register rd.
    ///
    /// `rd <- M[rs1 + sext(offset)][15:0]`
    LHU {
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Store Byte
    ///
    /// Store 8-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[7:0]`
    SB {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Store Half
    ///
    /// Store 16-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[15:0]`
    SH {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The 12-bit offset value sign-extended to an [i16].
        offset: i16,
    },

    /// # Store Word
    ///
    /// Store 32-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[rs1 + sext(offset)] = rs2[31:0]`
    SW {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The 12-bit offset value sign-extended to an [i16].
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
    },

    /// # Atomic CSR read set
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
    },

    /// # Atomic CSR read clear
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
        /// The destination register
        rd: Register,
        /// Source register 1.
        rs1: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
    },

    /// # Atomic CSR read write immediate
    ///
    /// Update the CSR using an XLEN-bit value obtained by zero-extending a
    /// 5-bit unsigned immediate (`uimm[4:0]`) field encoded in the rs1 field.
    ///
    /// `rd = CSRs[csr]; CSRs[csr] = zext(imm)`
    CSRRWI {
        /// The destination register
        rd: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
        /// The 5-bit immediate value zero-extended to a [u8].
        imm: u8,
    },

    /// # Atomic CSR read set immediate
    ///
    /// Set CSR bit using an XLEN-bit value obtained by zero-extending a 5-bit
    /// unsigned immediate (`uimm[4:0]`) field encoded in the rs1 field.
    ///
    /// `t = CSRs[csr]; CSRs[csr] = t | zext(imm); rd = t`
    CSRRSI {
        /// The destination register
        rd: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
        /// The 5-bit immediate value zero-extended to a [u8].
        imm: u8,
    },

    /// # Atomic CSR read clear immediate
    ///
    /// Clear CSR bit using an XLEN-bit value obtained by zero-extending a
    /// 5-bit unsigned immediate (`uimm[4:0]`) field encoded in the rs1 field.
    ///
    /// `t = CSRs[csr]; CSRs[csr] = t & ~zext(imm); rd = t`
    CSRRCI {
        /// The destination register
        rd: Register,
        /// The 12-bit csr address value zero-extended to an [u16].
        csr: u16,
        /// The 5-bit immediate value zero-extended to a [u8].
        imm: u8,
    },

    /// # Jump and link
    ///
    /// Jump to address and place return address in rd.
    ///
    /// `rd = pc + 4; pc += sext(offset)`
    ///
    /// _Note_: The JAL and JALR instructions will generate a misaligned
    /// instruction fetch exception if the target address is not aligned to a
    /// four-byte boundary.
    JAL {
        /// The destination register for the return address.
        ///
        /// The `RA` (`x1`) register is the usual return address register.
        /// However, `T0` (`x5`) can also be used as the alternative link
        /// register.
        ///
        /// The alternate link register supports calling millicode routines
        /// (e.g., those to save and restore registers in compressed code)
        /// while preserving the regular return address register. The register
        /// `T0` (`x5`) was chosen as the alternate link register as it maps to
        /// a temporary in the standard calling convention, and has an encoding
        /// that is only one bit different than the regular link register.
        ///
        /// Unconditional jumps set this to the `ZERO` (`x0`) register.
        rd: Register,
        /// The `21`-bit sign-extended offset. Adding this to the programme
        /// counter forms the jump target address.
        ///
        /// _Note_: This allows addressing on `2`-byte boundaries since as the
        /// least significant bit is always zero.
        offset: i32,
    },

    /// # Jump and link register
    ///
    /// Jump to address and place return address in rd.
    ///
    /// `t = pc + 4; pc = (rs1+sext(offset)) & ~1; rd = t`
    ///
    /// _Note_: The JAL and JALR instructions will generate a misaligned
    /// instruction fetch exception if the target address is not aligned to a
    /// four-byte boundary.
    JALR {
        /// The destination register for the return address.
        ///
        /// The `RA` (`x1`) register is the usual return address register.
        /// However, `T0` (`x5`) can also be used as the alternative link
        /// register.
        ///
        /// The alternate link register supports calling millicode routines
        /// (e.g., those to save and restore registers in compressed code)
        /// while preserving the regular return address register. The register
        /// `T0` (`x5`) was chosen as the alternate link register as it maps to
        /// a temporary in the standard calling convention, and has an encoding
        /// that is only one bit different than the regular link register.
        ///
        /// Unconditional jumps set this to the `ZERO` (`x0`) register.
        rd: Register,

        /// The source register.
        ///
        /// The target address is obtained by adding the `12`-bit signed
        /// `I`-immediate to the register `rs1`, then setting the
        /// least-significant bit of the result to zero.
        rs1: Register,
        /// The `12`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `12`-bit signed
        /// `I`-immediate to the register `rs1`, then setting the
        /// least-significant bit of the result to zero.
        offset: i16,
    },

    /// # Branch Equal
    ///
    /// Take the branch if registers rs1 and rs2 are equal.
    ///
    /// `if (rs1 == rs2) pc += sext(offset)`
    BEQ {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },

    /// # Branch Not Equal
    ///
    /// Take the branch if registers rs1 and rs2 are not equal.
    ///
    /// `if (rs1 != rs2) pc += sext(offset)`
    BNE {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },

    /// # Branch Less Than
    ///
    /// Take the branch if registers rs1 is less than rs2, using signed comparison.
    ///
    /// `if (rs1 < rs2) pc += sext(offset)`
    BLT {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },

    /// # Branch Greater Than or Equal
    ///
    /// Take the branch if registers rs1 is greater than or equal to rs2,
    /// using signed comparison.
    ///
    /// `if (rs1 >= rs2) pc += sext(offset)`
    BGE {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },

    /// # Branch Less Than Unsigned
    ///
    /// Take the branch if registers rs1 is less than rs2, using unsigned comparison.
    ///
    /// `if (rs1 <u rs2) pc += sext(offset)`
    BLTU {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },

    /// # Branch Greater Than or Equal Unsigned
    ///
    /// Take the branch if registers rs1 is greater than or equal to rs2,
    /// using unsigned comparison.
    ///
    /// `if (rs1 >=u rs2) pc += sext(offset)`
    BGEU {
        /// Source register 1.
        rs1: Register,
        /// Source register 2.
        rs2: Register,
        /// The `13`-bit sign-extended offset.
        ///
        /// The target address is obtained by adding the `13`-bit signed
        /// `B`-immediate to the programme counter.
        offset: i16,
    },
}

impl Instruction {
    /// Decode a [u32] as an [Instruction].
    ///
    /// Instructions in RISC-V are encoded using little endian byte order.
    /// Therefore, to avoid unexpected results, ensure the [u32] is little endian.
    #[inline]
    const fn decode(value: u32) -> Result<Self, Exception> {
        let instruction = match Instruction::op_code(value) {
            0b_0110111 => Instruction::LUI {
                rd: Rd::decode(value),
                imm: ImmU::decode(value),
            },
            0b_0010111 => Instruction::AUIPC {
                rd: Rd::decode(value),
                imm: ImmU::decode(value),
            },
            0b_0010011 => match Funct3::decode(value) {
                0b_000 => Instruction::ADDI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                0b_001 => Instruction::SLLI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    shamt: Shamt::decode(value),
                },
                0b_010 => Instruction::SLTI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                0b_011 => Instruction::SLTIU {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                0b_100 => Instruction::XORI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                0b_101 => match Funct6::decode(value) {
                    0b_000000 => Instruction::SRLI {
                        rd: Rd::decode(value),
                        rs1: Rs1::decode(value),
                        shamt: Shamt::decode(value),
                    },
                    0b_010000 => Instruction::SRAI {
                        rd: Rd::decode(value),
                        rs1: Rs1::decode(value),
                        shamt: Shamt::decode(value),
                    },
                    _ => return Err(Exception::UnimplementedInstruction(value)),
                },
                0b_110 => Instruction::ORI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                0b_111 => Instruction::ANDI {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    imm: ImmI::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_0110011 => match (Funct3::decode(value), Funct7::decode(value)) {
                (0b_000, 0b_0000000) => Instruction::ADD {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_000, 0b_0100000) => Instruction::SUB {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_001, 0b_0000000) => Instruction::SLL {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_010, 0b_0000000) => Instruction::SLT {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_011, 0b_0000000) => Instruction::SLTU {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_100, 0b_0000000) => Instruction::XOR {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_101, 0b_0000000) => Instruction::SRL {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_101, 0b_0100000) => Instruction::SRA {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_110, 0b_0000000) => Instruction::OR {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                (0b_111, 0b_0000000) => Instruction::AND {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_0000011 => match Funct3::decode(value) {
                0b_000 => Instruction::LB {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                0b_001 => Instruction::LH {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                0b_010 => Instruction::LW {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                0b_100 => Instruction::LBU {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                0b_101 => Instruction::LHU {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_0100011 => match Funct3::decode(value) {
                0b_000 => Instruction::SB {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: SImmI::decode(value),
                },
                0b_001 => Instruction::SH {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: SImmI::decode(value),
                },
                0b_010 => Instruction::SW {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: SImmI::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_1110011 => match Funct3::decode(value) {
                0b_001 => Instruction::CSRRW {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    csr: Csr::decode(value),
                },
                0b_010 => Instruction::CSRRS {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    csr: Csr::decode(value),
                },
                0b_011 => Instruction::CSRRC {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    csr: Csr::decode(value),
                },
                0b_101 => Instruction::CSRRWI {
                    rd: Rd::decode(value),
                    imm: CsrImm::decode(value),
                    csr: Csr::decode(value),
                },
                0b_110 => Instruction::CSRRSI {
                    rd: Rd::decode(value),
                    imm: CsrImm::decode(value),
                    csr: Csr::decode(value),
                },
                0b_111 => Instruction::CSRRCI {
                    rd: Rd::decode(value),
                    imm: CsrImm::decode(value),
                    csr: Csr::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_1101111 => Instruction::JAL {
                rd: Rd::decode(value),
                offset: JImm::decode(value),
            },
            0b_1100011 => match Funct3::decode(value) {
                0b_000 => Instruction::BEQ {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                0b_001 => Instruction::BNE {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                0b_100 => Instruction::BLT {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                0b_101 => Instruction::BGE {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                0b_110 => Instruction::BLTU {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                0b_111 => Instruction::BGEU {
                    rs1: Rs1::decode(value),
                    rs2: Rs2::decode(value),
                    offset: BImm::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            0b_1100111 => match Funct3::decode(value) {
                0b_000 => Instruction::JALR {
                    rd: Rd::decode(value),
                    rs1: Rs1::decode(value),
                    offset: ImmI::decode(value),
                },
                _ => return Err(Exception::UnimplementedInstruction(value)),
            },
            _ => return Err(Exception::UnimplementedInstruction(value)),
        };
        Ok(instruction)
    }

    /// Encode an [Instruction] as a little endian [u32].
    #[inline]
    pub(crate) const fn encode(self) -> u32 {
        match self {
            Instruction::LUI { rd, imm } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0110111) + types::U::encode(rd, imm)
            }
            Instruction::AUIPC { rd, imm } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0010111) + types::U::encode(rd, imm)
            }
            Instruction::ADDI { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::SLTI { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_010_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::SLTIU { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_011_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::XORI { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_100_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::ORI { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_110_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::ANDI { rd, rs1, imm } => {
                u32::from_le(0b_0000000_00000_00000_111_00000_0010011)
                    + types::I::encode(rd, rs1, imm)
            }
            Instruction::SLLI { rd, rs1, shamt } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_0010011)
                    + types::I::encode_shamt(rd, rs1, shamt)
            }
            Instruction::SRLI { rd, rs1, shamt } => {
                u32::from_le(0b_0000000_00000_00000_101_00000_0010011)
                    + types::I::encode_shamt(rd, rs1, shamt)
            }
            Instruction::SRAI { rd, rs1, shamt } => {
                u32::from_le(0b_0100000_00000_00000_101_00000_0010011)
                    + types::I::encode_shamt(rd, rs1, shamt)
            }
            Instruction::ADD { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SUB { rd, rs1, rs2 } => {
                u32::from_le(0b_0100000_00000_00000_000_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SLL { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SLT { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_010_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SLTU { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_011_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::XOR { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_100_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SRL { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_101_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::SRA { rd, rs1, rs2 } => {
                u32::from_le(0b_0100000_00000_00000_101_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::OR { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_110_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::AND { rd, rs1, rs2 } => {
                u32::from_le(0b_0000000_00000_00000_111_00000_0110011)
                    + types::R::encode(rd, rs1, rs2)
            }
            Instruction::LB { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0000011)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::LH { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_0000011)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::LW { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_010_00000_0000011)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::LBU { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_100_00000_0000011)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::LHU { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_101_00000_0000011)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::SB { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_0100011)
                    + types::S::encode(rs1, rs2, offset)
            }
            Instruction::SH { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_0100011)
                    + types::S::encode(rs1, rs2, offset)
            }
            Instruction::SW { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_010_00000_0100011)
                    + types::S::encode(rs1, rs2, offset)
            }
            Instruction::CSRRW { rd, rs1, csr } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_1110011)
                    + types::I::encode_csr(rd, rs1, csr)
            }
            Instruction::CSRRS { rd, rs1, csr } => {
                u32::from_le(0b_0000000_00000_00000_010_00000_1110011)
                    + types::I::encode_csr(rd, rs1, csr)
            }
            Instruction::CSRRC { rd, rs1, csr } => {
                u32::from_le(0b_0000000_00000_00000_011_00000_1110011)
                    + types::I::encode_csr(rd, rs1, csr)
            }
            Instruction::CSRRWI { rd, csr, imm } => {
                u32::from_le(0b_0000000_00000_00000_101_00000_1110011)
                    + types::I::encode_csri(rd, imm, csr)
            }
            Instruction::CSRRSI { rd, csr, imm } => {
                u32::from_le(0b_0000000_00000_00000_110_00000_1110011)
                    + types::I::encode_csri(rd, imm, csr)
            }
            Instruction::CSRRCI { rd, csr, imm } => {
                u32::from_le(0b_0000000_00000_00000_111_00000_1110011)
                    + types::I::encode_csri(rd, imm, csr)
            }
            Instruction::JAL { rd, offset } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_1101111)
                    + types::J::encode(rd, offset)
            }
            Instruction::JALR { rd, rs1, offset } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_1100111)
                    + types::I::encode(rd, rs1, offset)
            }
            Instruction::BEQ { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_000_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
            Instruction::BNE { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_001_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
            Instruction::BLT { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_100_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
            Instruction::BGE { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_101_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
            Instruction::BLTU { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_110_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
            Instruction::BGEU { rs1, rs2, offset } => {
                u32::from_le(0b_0000000_00000_00000_111_00000_1100011)
                    + types::B::encode(rs1, rs2, offset)
            }
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = Exception;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Instruction::decode(value)
    }
}

impl Instruction {
    /// Extract the op code from the [u32].
    #[inline]
    const fn op_code(value: u32) -> u8 {
        (value & OPP_MASK) as u8
    }
}

/// The bit mask to extract the instructions op code from a [u32].
const OPP_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_00000_1111111);

#[cfg(test)]
mod test {
    use crate::{instructions::Instruction, registers::Register};
    use pretty_assertions::assert_eq;

    impl Instruction {
        fn from(value: u32) -> Self {
            Self::try_from(value).expect("Unimplemented")
        }
    }

    #[test]
    fn lui_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_101_11000_0110111)),
            Instruction::LUI {
                rd: Register::S8,
                imm: 0x48A65
            }
        );
    }

    #[test]
    fn encode_lui() {
        assert_eq!(
            Instruction::LUI {
                rd: Register::S8,
                imm: 0x48A65
            }
            .encode(),
            u32::from_le(0b_0100100_01010_01100_101_11000_0110111)
        );
    }

    #[test]
    fn auipc_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0100100_01010_01100_111_11100_0010111)),
            Instruction::AUIPC {
                rd: Register::T3,
                imm: 0x48A67
            }
        );
    }

    #[test]
    fn encode_auipc() {
        assert_eq!(
            Instruction::AUIPC {
                rd: Register::T3,
                imm: 0x48A67
            }
            .encode(),
            u32::from_le(0b_0100100_01010_01100_111_11100_0010111),
        );
    }

    #[test]
    fn addi_from_u32() {
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
    fn encode_addi() {
        assert_eq!(
            Instruction::ADDI {
                rd: Register::RA,
                rs1: Register::ZERO,
                imm: 32
            }
            .encode(),
            u32::from_le(0b_0000001_00000_00000_000_00001_0010011),
        );
    }

    #[test]
    fn slti_from_u32() {
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
    fn encode_slti() {
        assert_eq!(
            Instruction::SLTI {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 32
            }
            .encode(),
            u32::from_le(0b_0000001_00000_00100_010_00011_0010011),
        );
    }

    #[test]
    fn sltiu_from_u32() {
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
    fn encode_sltiu() {
        assert_eq!(
            Instruction::SLTIU {
                rd: Register::GP,
                rs1: Register::TP,
                imm: 96
            }
            .encode(),
            u32::from_le(0b_0000011_00000_00100_011_00011_0010011),
        );
    }

    #[test]
    fn xori_from_u32() {
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
    fn encode_xori() {
        assert_eq!(
            Instruction::XORI {
                rd: Register::A1,
                rs1: Register::A2,
                imm: -8
            }
            .encode(),
            u32::from_le(0b_1111111_11000_01100_100_01011_0010011),
        );
    }

    #[test]
    fn ori_from_u32() {
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
    fn encode_ori() {
        assert_eq!(
            Instruction::ORI {
                rd: Register::S3,
                rs1: Register::A3,
                imm: -24
            }
            .encode(),
            u32::from_le(0b_1111111_01000_01101_110_10011_0010011),
        );
    }

    #[test]
    fn andi_from_u32() {
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
    fn encode_andi() {
        assert_eq!(
            Instruction::ANDI {
                rd: Register::A5,
                rs1: Register::T3,
                imm: 72
            }
            .encode(),
            u32::from_le(0b_0000010_01000_11100_111_01111_0010011),
        );
    }

    #[test]
    fn slli_from_u32() {
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
    fn encode_slli() {
        assert_eq!(
            Instruction::SLLI {
                rd: Register::A3,
                rs1: Register::S2,
                shamt: 10
            }
            .encode(),
            u32::from_le(0b_0000000_01010_10010_001_01101_0010011),
        );
    }

    #[test]
    fn srli_from_u32() {
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
    fn encode_srli() {
        assert_eq!(
            Instruction::SRLI {
                rd: Register::A4,
                rs1: Register::S3,
                shamt: 42
            }
            .encode(),
            u32::from_le(0b_0000001_01010_10011_101_01110_0010011),
        );
    }

    #[test]
    fn srai_from_u32() {
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
    fn encode_srai() {
        assert_eq!(
            Instruction::SRAI {
                rd: Register::A6,
                rs1: Register::S4,
                shamt: 26
            }
            .encode(),
            u32::from_le(0b_0100000_11010_10100_101_10000_0010011),
        );
    }

    #[test]
    fn add_from_u32() {
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
    fn encode_add() {
        assert_eq!(
            Instruction::ADD {
                rd: Register::SP,
                rs1: Register::A1,
                rs2: Register::A3,
            }
            .encode(),
            u32::from_le(0b_0000000_01101_01011_000_00010_0110011),
        );
    }

    #[test]
    fn sub_from_u32() {
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
    fn encode_sub() {
        assert_eq!(
            Instruction::SUB {
                rd: Register::SP,
                rs1: Register::S11,
                rs2: Register::T4,
            }
            .encode(),
            u32::from_le(0b_0100000_11101_11011_000_00010_0110011),
        );
    }

    #[test]
    fn sll_from_u32() {
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
    fn encode_sll() {
        assert_eq!(
            Instruction::SLL {
                rd: Register::S4,
                rs1: Register::S5,
                rs2: Register::S6,
            }
            .encode(),
            u32::from_le(0b_0000000_10110_10101_001_10100_0110011),
        );
    }

    #[test]
    fn stl_from_u32() {
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
    fn encode_stl() {
        assert_eq!(
            Instruction::SLT {
                rd: Register::T1,
                rs1: Register::S3,
                rs2: Register::T3,
            }
            .encode(),
            u32::from_le(0b_0000000_11100_10011_010_00110_0110011),
        );
    }

    #[test]
    fn stlu_from_u32() {
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
    fn encode_stlu() {
        assert_eq!(
            Instruction::SLTU {
                rd: Register::A4,
                rs1: Register::A7,
                rs2: Register::S8,
            }
            .encode(),
            u32::from_le(0b_0000000_11000_10001_011_01110_0110011),
        );
    }

    #[test]
    fn xor_from_u32() {
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
    fn encode_xor() {
        assert_eq!(
            Instruction::XOR {
                rd: Register::T0,
                rs1: Register::A0,
                rs2: Register::T2,
            }
            .encode(),
            u32::from_le(0b_0000000_00111_01010_100_00101_0110011),
        );
    }

    #[test]
    fn srl_from_u32() {
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
    fn encode_srl() {
        assert_eq!(
            Instruction::SRL {
                rd: Register::S7,
                rs1: Register::S8,
                rs2: Register::S9,
            }
            .encode(),
            u32::from_le(0b_0000000_11001_11000_101_10111_0110011),
        );
    }

    #[test]
    fn sra_from_u32() {
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
    fn encode_sra() {
        assert_eq!(
            Instruction::SRA {
                rd: Register::S10,
                rs1: Register::S11,
                rs2: Register::T3,
            }
            .encode(),
            u32::from_le(0b_0100000_11100_11011_101_11010_0110011),
        );
    }

    #[test]
    fn or_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_11100_01001_110_01000_0110011)),
            Instruction::OR {
                rd: Register::S0,
                rs1: Register::S1,
                rs2: Register::T3,
            }
        );
    }

    #[test]
    fn encode_or() {
        assert_eq!(
            Instruction::OR {
                rd: Register::S0,
                rs1: Register::S1,
                rs2: Register::T3,
            }
            .encode(),
            u32::from_le(0b_0000000_11100_01001_110_01000_0110011),
        );
    }

    #[test]
    fn and_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000000_11111_11110_111_11101_0110011)),
            Instruction::AND {
                rd: Register::T4,
                rs1: Register::T5,
                rs2: Register::T6,
            }
        );
    }

    #[test]
    fn encode_and() {
        assert_eq!(
            Instruction::AND {
                rd: Register::T4,
                rs1: Register::T5,
                rs2: Register::T6,
            }
            .encode(),
            u32::from_le(0b_0000000_11111_11110_111_11101_0110011),
        );
    }

    #[test]
    fn lb_from_u32() {
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
    fn encode_lb() {
        assert_eq!(
            Instruction::LB {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 57,
            }
            .encode(),
            u32::from_le(0b_0000001_11001_01100_000_11100_0000011),
        );
    }

    #[test]
    fn lh_from_u32() {
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
    fn encode_lh() {
        assert_eq!(
            Instruction::LH {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 58,
            }
            .encode(),
            u32::from_le(0b_0000001_11010_01100_001_11100_0000011),
        );
    }

    #[test]
    fn lw_from_u32() {
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
    fn encode_lw() {
        assert_eq!(
            Instruction::LW {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 56,
            }
            .encode(),
            u32::from_le(0b_0000001_11000_01100_010_11100_0000011),
        );
    }

    #[test]
    fn lbu_from_u32() {
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
    fn encode_lbu() {
        assert_eq!(
            Instruction::LBU {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 59,
            }
            .encode(),
            u32::from_le(0b_0000001_11011_01100_100_11100_0000011),
        );
    }

    #[test]
    fn lhu_from_u32() {
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
    fn encode_lhu() {
        assert_eq!(
            Instruction::LHU {
                rd: Register::T3,
                rs1: Register::A2,
                offset: 60,
            }
            .encode(),
            u32::from_le(0b_0000001_11100_01100_101_11100_0000011),
        );
    }

    #[test]
    fn sb_from_u32() {
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
    fn encode_sb() {
        assert_eq!(
            Instruction::SB {
                rs1: Register::A3,
                rs2: Register::T4,
                offset: 61,
            }
            .encode(),
            u32::from_le(0b_0000001_11101_01101_000_11101_0100011),
        );
    }

    #[test]
    fn sh_from_u32() {
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
    fn encode_sh() {
        assert_eq!(
            Instruction::SH {
                rs1: Register::A3,
                rs2: Register::T4,
                offset: 62,
            }
            .encode(),
            u32::from_le(0b_0000001_11101_01101_001_11110_0100011),
        );
    }

    #[test]
    fn sw_from_u32() {
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
    fn encode_sw() {
        assert_eq!(
            Instruction::SW {
                rs1: Register::A3,
                rs2: Register::T6,
                offset: 63,
            }
            .encode(),
            u32::from_le(0b_0000001_11111_01101_010_11111_0100011),
        );
    }

    #[test]
    fn csrrw_from_u32() {
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
    fn encode_csrrw() {
        assert_eq!(
            Instruction::CSRRW {
                rd: Register::S11,
                rs1: Register::A5,
                csr: 319,
            }
            .encode(),
            u32::from_le(0b_0001001_11111_01111_001_11011_1110011),
        );
    }

    #[test]
    fn csrrs_from_u32() {
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
    fn encode_csrrs() {
        assert_eq!(
            Instruction::CSRRS {
                rd: Register::S3,
                rs1: Register::A1,
                csr: 1343,
            }
            .encode(),
            u32::from_le(0b_0101001_11111_01011_010_10011_1110011),
        );
    }

    #[test]
    fn csrrc_from_u32() {
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
    fn encode_csrrc() {
        assert_eq!(
            Instruction::CSRRC {
                rd: Register::S7,
                rs1: Register::S1,
                csr: 3391,
            }
            .encode(),
            u32::from_le(0b_1101001_11111_01001_011_10111_1110011),
        );
    }

    #[test]
    fn csrrwi_from_u32() {
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
    fn encode_csrrwi() {
        assert_eq!(
            Instruction::CSRRWI {
                rd: Register::S5,
                imm: 9,
                csr: 59,
            }
            .encode(),
            u32::from_le(0b_0000001_11011_01001_101_10101_1110011),
        );
    }

    #[test]
    fn csrrsi_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_11011_110_10100_1110011)),
            Instruction::CSRRSI {
                rd: Register::S4,
                imm: 27,
                csr: 42,
            }
        );
    }

    #[test]
    fn encode_csrrsi() {
        assert_eq!(
            Instruction::CSRRSI {
                rd: Register::S4,
                imm: 27,
                csr: 42,
            }
            .encode(),
            u32::from_le(0b_0000001_01010_11011_110_10100_1110011),
        );
    }

    #[test]
    fn csrrci_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000011_11011_11001_111_10001_1110011)),
            Instruction::CSRRCI {
                rd: Register::A7,
                imm: 25,
                csr: 123,
            }
        );
    }

    #[test]
    fn encode_csrrci() {
        assert_eq!(
            Instruction::CSRRCI {
                rd: Register::A7,
                imm: 25,
                csr: 123,
            }
            .encode(),
            u32::from_le(0b_0000011_11011_11001_111_10001_1110011),
        );
    }

    #[test]
    fn jal_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_00000_000_00001_1101111)),
            Instruction::JAL {
                rd: Register::RA,
                offset: 42,
            }
        );
    }

    #[test]
    fn encode_jal() {
        assert_eq!(
            Instruction::JAL {
                rd: Register::T0,
                offset: 2090,
            }
            .encode(),
            u32::from_le(0b_0000001_01011_00000_000_00101_1101111),
        );
    }

    #[test]
    fn jalr_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_01000_000_00001_1100111)),
            Instruction::JALR {
                rd: Register::RA,
                rs1: Register::FP,
                offset: 42,
            }
        );
    }

    #[test]
    fn encode_jalr() {
        assert_eq!(
            Instruction::JALR {
                rd: Register::T0,
                rs1: Register::A0,
                offset: 554,
            }
            .encode(),
            u32::from_le(0b_0010001_01010_01010_000_00101_1100111),
        );
    }

    #[test]
    fn beq_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01010_01000_000_00001_1100011)),
            Instruction::BEQ {
                rs1: Register::FP,
                rs2: Register::A0,
                offset: 2080,
            }
        );
    }

    #[test]
    fn encode_beq() {
        assert_eq!(
            Instruction::BEQ {
                rs1: Register::FP,
                rs2: Register::A0,
                offset: -2016,
            }
            .encode(),
            u32::from_le(0b_1000001_01010_01000_000_00001_1100011),
        );
    }

    #[test]
    fn bne_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01011_01001_001_00101_1100011)),
            Instruction::BNE {
                rs1: Register::S1,
                rs2: Register::A1,
                offset: 2084,
            }
        );
    }

    #[test]
    fn encode_bne() {
        assert_eq!(
            Instruction::BNE {
                rs1: Register::S1,
                rs2: Register::A1,
                offset: -2014,
            }
            .encode(),
            u32::from_le(0b_1000001_01011_01001_001_00011_1100011),
        );
    }

    #[test]
    fn blt_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01111_01101_100_00001_1100011)),
            Instruction::BLT {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: 2080,
            }
        );
    }

    #[test]
    fn encode_blt() {
        assert_eq!(
            Instruction::BLT {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: -2016,
            }
            .encode(),
            u32::from_le(0b_1000001_01111_01101_100_00001_1100011),
        );
    }

    #[test]
    fn bge_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01111_01101_101_00001_1100011)),
            Instruction::BGE {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: 2080,
            }
        );
    }

    #[test]
    fn encode_bge() {
        assert_eq!(
            Instruction::BGE {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: -2016,
            }
            .encode(),
            u32::from_le(0b_1000001_01111_01101_101_00001_1100011),
        );
    }

    #[test]
    fn bltu_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01111_01101_110_00001_1100011)),
            Instruction::BLTU {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: 2080,
            }
        );
    }

    #[test]
    fn encode_bltu() {
        assert_eq!(
            Instruction::BLTU {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: -2016,
            }
            .encode(),
            u32::from_le(0b_1000001_01111_01101_110_00001_1100011),
        );
    }

    #[test]
    fn bgeu_from_u32() {
        assert_eq!(
            Instruction::from(u32::from_le(0b_0000001_01111_01101_111_00001_1100011)),
            Instruction::BGEU {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: 2080,
            }
        );
    }

    #[test]
    fn encode_bgeu() {
        assert_eq!(
            Instruction::BGEU {
                rs1: Register::A3,
                rs2: Register::A5,
                offset: -2016,
            }
            .encode(),
            u32::from_le(0b_1000001_01111_01101_111_00001_1100011),
        );
    }
}
