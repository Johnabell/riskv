//! Debug printer for instructions
#![allow(clippy::unusual_byte_groupings)]

/// The binary representation of an instruction
#[repr(transparent)]
pub(crate) struct BinInstruction(u32);

impl BinInstruction {
    /// The bit mask to extract the `opcode`.
    const OPCODE_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_00000_1111111);
    /// The bit mask to extract the destination register `rd`.
    const RD_MASK: u32 = u32::from_le(0b_0000000_00000_00000_000_11111_0000000);
    /// The number of bits `rd` is offset in the instruction
    const RD_SHIFT: usize = 7;
    /// The bit mask to extract `func3`.
    const FUNC3_MASK: u32 = u32::from_le(0b_0000000_00000_00000_111_00000_0000000);
    /// The number of bits `func3` is offset in the instruction
    const FUNC3_SHIFT: usize = 12;
    /// The bit mask to extract the source register `rs1`.
    const RS1_MASK: u32 = u32::from_le(0b_0000000_00000_11111_000_00000_0000000);
    /// The number of bits `rs1` is offset in the instruction
    const RS1_SHIFT: usize = 15;
    /// The bit mask to extract the source register `rs2`.
    const RS2_MASK: u32 = u32::from_le(0b_0000000_11111_00000_000_00000_0000000);
    /// The number of bits `rs2` is offset in the instruction
    const RS2_SHIFT: usize = 20;
    /// The bit mask to extract `func7`.
    const FUNC7_MASK: u32 = u32::from_le(0b_1111111_00000_00000_000_00000_0000000);
    /// The number of bits `func7` is offset in the instruction
    const FUNC7_SHIFT: usize = 25;

    /// The first 7-bits of the instruction known as the `opcode`.
    #[inline]
    fn opcode(&self) -> u32 {
        self.0 & Self::OPCODE_MASK
    }

    /// Bits `7 - 11` of the instruction often used for the destination register `rd`.
    #[inline]
    fn rd(&self) -> u32 {
        (self.0 & Self::RD_MASK) >> Self::RD_SHIFT
    }

    /// Bits `12 - 14` of the instruction usually referred to as `func3`.
    #[inline]
    fn func3(&self) -> u32 {
        (self.0 & Self::FUNC3_MASK) >> Self::FUNC3_SHIFT
    }

    /// Bits `15 - 19` of the instruction often used for a source register `rs1`.
    #[inline]
    fn rs1(&self) -> u32 {
        (self.0 & Self::RS1_MASK) >> Self::RS1_SHIFT
    }

    /// Bits `20 - 24` of the instruction often used for a source register `rs2`.
    #[inline]
    fn rs2(&self) -> u32 {
        (self.0 & Self::RS2_MASK) >> Self::RS2_SHIFT
    }

    /// Bits `25 - 31` of the instruction usually referred to as `func7`.
    #[inline]
    fn func7(&self) -> u32 {
        (self.0 & Self::FUNC7_MASK) >> Self::FUNC7_SHIFT
    }
}

impl From<u32> for BinInstruction {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<BinInstruction> for u32 {
    #[inline]
    fn from(value: BinInstruction) -> Self {
        value.0
    }
}

impl std::fmt::Debug for BinInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "0b_{func7:07b}_{rs2:05b}_{rs1:05b}_{func3:03b}_{rd:05b}_{opcode:07b}",
            func7 = self.func7(),
            rs2 = self.rs2(),
            rs1 = self.rs1(),
            func3 = self.func3(),
            rd = self.rd(),
            opcode = self.opcode(),
        ))
    }
}

#[allow(unused)]
pub(crate) fn dbg_inst(raw_instruction: u32) -> u32 {
    eprintln!("{:?}", BinInstruction(raw_instruction));
    raw_instruction
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug_print() {
        dbg!(20);
        assert_eq!(
            "0b_0110110_10101_10110_011_10111_0010011",
            format!(
                "{:?}",
                BinInstruction(0b_0110110_10101_10110_011_10111_0010011)
            )
        );
    }
}
