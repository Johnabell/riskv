# riskv (pronounced 'riskvey')

A RISC-V interpreter written in Rust.

## Status

This project is in early stages and it primarily being embarked upon as a learning exercise.
Feel free to contribute and get involved if you would like to learn more about assembly and RISC-V.

## TODO

<details open>
  <summary>
    - [ ] Implement RV32I:
  </summary>

  - [x] LUI
  - [x] AUIPC
  - [x] ADDI
  - [x] SLTI
  - [x] SLTUI
  - [x] XORI
  - [x] ORI
  - [x] ANDI
  - [x] SLLI
  - [x] SRLI
  - [x] SRAI
  - [x] ADD
  - [x] SUB
  - [x] SLL
  - [x] SLT
  - [x] SLTU
  - [x] XOR
  - [x] SRL
  - [x] SRA
  - [x] OR
  - [x] AND
  - [ ] FENCE
  - [ ] FENCE.I
  - [x] CSRRW
  - [x] CSRRS
  - [x] CSRRC
  - [X] CSRRWI
  - [X] CSRRSI
  - [X] CSRRCI
  - [ ] ECALL
  - [ ] EBREAK
  - [X] LB
  - [X] LH
  - [X] LW
  - [X] LBU
  - [X] LHU
  - [X] SB
  - [X] SH
  - [X] SW
  - [x] JAL
  - [x] JALR
  - [ ] BEQ
  - [ ] BNE
  - [ ] BLT
  - [ ] BGE
  - [ ] BLTU
  - [ ] BGEU

</details>

<details open>
  <summary>
    - [ ] Implement privileged RV32I:
  </summary>

  - [ ] URET
  - [ ] SRET
  - [ ] MRET
  - [ ] WFI
  - [ ] SFENCE.VMA

</details>

## Provisional plan

- Implement basic model of the processor, instructions, stack and heap (based on a useful subset of all the instructions)
- Implement a parser for RISC-V assembly files
- Interpret and run RISC-V assembly files

## Possible future additions

- A graphical UI for inspecting the current processor state during the execution of a programme
- An assembler to create native RISK-V binaries

## Code of conduct

We follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

The moderation team consists of:

* John Bell (@johnabell)
* Andy Balaam (@andybalaam)

We welcome more members: if you would like to join the moderation team, please contact John Bell.

## Licence

The project is licensed under the#
[GNU Affero General Public License v3.0](https://github.com/Johnabell/riskv/blob/master/LICENSE).

## Useful links

- [RISC-V assembly for beginers](https://medium.com/swlh/risc-v-assembly-for-beginners-387c6cd02c49)
- [RISC-V instruction set cheatsheet](https://itnext.io/risc-v-instruction-set-cheatsheet-70961b4bbe8)
- [RISC-V assembly course](https://web.eecs.utk.edu/~smarz1/courses/ece356/notes/assembly/)
- [RISC-V examples](https://github.com/takenobu-hs/cpu-assembly-examples/tree/master/riscv/linux)
- [RISC-V Assembly Programmer's Manual](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md)
- [RISC-V Visual Emulator](https://eseo-tech.github.io/emulsiV/)
- [RISC-V Instructions reference](https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html)
- [The RISC-V Instruction Set Manual Volume I: Unprivileged ISA (pdf)](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf)
