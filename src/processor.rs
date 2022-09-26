use crate::csr::CSR;
use crate::memory::Memory;
use crate::registers::Registers;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Processor<R, CSRs: CSR<Register = R>> {
    pub(crate) registers: Registers<R>,
    // Programme Counter
    pub(crate) pc: R,
    pub(crate) csrs: CSRs,
    pub(crate) memory: Memory,
}
