use crate::registers::Register;

use super::{
    csr::Csr, csr_imm::CsrImm, immi::ImmI, immu::ImmU, rd::Rd, rs1::Rs1, rs2::Rs2, shamt::Shamt,
    simmi::SImmI,
};

pub(super) struct R;

impl R {
    pub(super) const fn encode(rd: Register, rs1: Register, rs2: Register) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Rs2::encode(rs2)
    }
}

pub(super) struct I;

impl I {
    pub(super) const fn encode(rd: Register, rs1: Register, imm: i16) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + ImmI::encode(imm)
    }

    pub(super) const fn encode_csr(rd: Register, rs1: Register, csr: u16) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Csr::encode(csr)
    }

    pub(super) const fn encode_csri(rd: Register, imm: u8, csr: u16) -> u32 {
        Rd::encode(rd) + CsrImm::encode(imm) + Csr::encode(csr)
    }

    pub(super) const fn encode_shamt(rd: Register, rs1: Register, shamt: u8) -> u32 {
        Rd::encode(rd) + Rs1::encode(rs1) + Shamt::encode(shamt)
    }
}

pub(super) struct U;

impl U {
    pub(super) const fn encode(rd: Register, imm: i32) -> u32 {
        Rd::encode(rd) + ImmU::encode(imm)
    }
}

pub(super) struct S;

impl S {
    pub(super) const fn encode(rs1: Register, rs2: Register, simmi: i16) -> u32 {
        Rs1::encode(rs1) + Rs2::encode(rs2) + SImmI::encode(simmi)
    }
}
