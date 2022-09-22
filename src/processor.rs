use crate::memory::Memory;
use crate::registers::Registers;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Processor<R> {
    pub(crate) registers: Registers<R>,
    // Programme Counter
    pub(crate) pc: R,
    pub(crate) memory: Memory,
}
