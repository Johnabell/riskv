//! RISC-V defines a number of different instruction types outlined below.
//!
//! | Type | Description                      |
//! | ---- | -------------------------------- |
//! | R    | Register type instruction        |
//! | I    | Immediate type instruction       |
//! | U    | Upper Immediate type instruction |
//! | S    | Store type instruction           |
//! | B    | Branch type instruction          |
//! | J    | Jump type instruction            |
//!
//! This module includes helpers for encoding instructions of the different types.
use crate::registers::Register;

use super::{
    csr::Csr, csr_imm::CsrImm, immi::ImmI, immu::ImmU, rd::Rd, rs1::Rs1, rs2::Rs2, shamt::Shamt,
    simmi::SImmI,
};

/// `R`-type instructions - Register type instructions.
pub(super) struct R;

impl R {
    /// Encode the source and destination registers as an `R`-type instruction.
    pub(super) const fn encode(rd: Register, rs1: Register, rs2: Register) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Rs2::encode(rs2)
    }
}

/// `I`-type instructions - Immediate type instructions.
pub(super) struct I;

impl I {
    /// Encode the source and destination registers, and immediate value as an
    /// `I`-type instruction.
    pub(super) const fn encode(rd: Register, rs1: Register, imm: i16) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + ImmI::encode(imm)
    }

    /// Encode the source and destination registers, and `CSR` value as an
    /// `I`-type instruction.
    pub(super) const fn encode_csr(rd: Register, rs1: Register, csr: u16) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Csr::encode(csr)
    }

    /// Encode the destination register, immediate value, and `CSR` value as an
    /// `I`-type instruction.
    pub(super) const fn encode_csri(rd: Register, imm: u8, csr: u16) -> u32 {
        Rd::encode(rd) + CsrImm::encode(imm) + Csr::encode(csr)
    }

    /// Encode the source and destination registers, and shift amount as an
    /// `I`-type instruction.
    pub(super) const fn encode_shamt(rd: Register, rs1: Register, shamt: u8) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Shamt::encode(shamt)
    }
}

/// `U`-type instructions - Upper immediate type instructions.
pub(super) struct U;

impl U {
    /// Encode the destination register and immediate value as an `U`-type
    /// instruction.
    pub(super) const fn encode(rd: Register, imm: i32) -> u32 {
        Rd::encode(rd) + ImmU::encode(imm)
    }
}

/// `S`-type instructions - Store type instructions.
pub(super) struct S;

impl S {
    /// Encode the source registers and immediate value as an `S`-type instruction.
    pub(super) const fn encode(rs1: Register, rs2: Register, simmi: i16) -> u32 {
        Rs1::encode(rs1) + Rs2::encode(rs2) + SImmI::encode(simmi)
    }
}
