//! RISKV - An implementation of a RISC-V emulator
#![warn(unused_crate_dependencies)]
#![deny(missing_docs, clippy::missing_docs_in_private_items)]

pub mod csr;
pub mod instruction_set;
mod instructions;
mod integer;
mod memory;
pub mod processor;
mod registers;
#[cfg(any(test, doc))]
mod test;
