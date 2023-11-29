//! The processors registers.
//!
//! A RISC-V processor has 32 integer registers, referred to either as `x0-x31`
//! or by their ABI name.
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// Struct representing the RISK-V processor registers.
///
/// The processor contains the following registers.
///
/// | rx | rx' | Register | ABI Name | Description                          | Saved by |
/// | -- | --- | -------- | -------- | ------------------------------------ | -------- |
/// | 0  | -   | x0       | zero     | hardwired zero                       | -        |
/// | 1  | -   | x1       | ra       | return address                       | caller   |
/// | 2  | -   | x2       | sp       | stack pointer                        | callee   |
/// | 3  | -   | x3       | gp       | global pointer                       | -        |
/// | 4  | -   | x4       | tp       | thread pointer                       | -        |
/// | 5  | -   | x5       | t0       | temporary register 0                 | caller   |
/// | 6  | -   | x6       | t1       | temporary register 1                 | caller   |
/// | 7  | -   | x7       | t2       | temporary register 2                 | caller   |
/// | 8  | 0   | x8       | s0 / fp  | saved register 0 / frame pointer     | callee   |
/// | 9  | 1   | x9       | s1       | saved register 1                     | callee   |
/// | 10 | 2   | x10      | a0       | function argument 0 / return value 0 | caller   |
/// | 11 | 3   | x11      | a1       | function argument 1 / return value 1 | caller   |
/// | 12 | 4   | x12      | a2       | function argument 2                  | caller   |
/// | 13 | 5   | x13      | a3       | function argument 3                  | caller   |
/// | 14 | 6   | x14      | a4       | function argument 4                  | caller   |
/// | 15 | 7   | x15      | a5       | function argument 5                  | caller   |
/// | 16 | -   | x16      | a6       | function argument 6                  | caller   |
/// | 17 | -   | x17      | a7       | function argument 7                  | caller   |
/// | 18 | -   | x18      | s2       | saved register 2                     | callee   |
/// | 19 | -   | x19      | s3       | saved register 3                     | callee   |
/// | 20 | -   | x20      | s4       | saved register 4                     | callee   |
/// | 21 | -   | x21      | s5       | saved register 5                     | callee   |
/// | 22 | -   | x22      | s6       | saved register 6                     | callee   |
/// | 23 | -   | x23      | s7       | saved register 7                     | callee   |
/// | 24 | -   | x24      | s8       | saved register 8                     | callee   |
/// | 25 | -   | x25      | s9       | saved register 9                     | callee   |
/// | 26 | -   | x26      | s10      | saved register 10                    | callee   |
/// | 27 | -   | x27      | s11      | saved register 11                    | callee   |
/// | 28 | -   | x28      | t3       | temporary register 3                 | caller   |
/// | 29 | -   | x29      | t4       | temporary register 4                 | caller   |
/// | 30 | -   | x30      | t5       | temporary register 5                 | caller   |
/// | 31 | -   | x31      | t6       | temporary register 6                 | caller   |
#[derive(Default, PartialEq, Eq)]
pub(super) struct Registers<T> {
    /// The zero register.
    ///
    /// This register will always yield zero. Setting the destination register
    /// of a specific instruction will discard the result.
    ///
    /// In particular instructions that usually trigger read read side effects
    /// should not be triggered if their destination register is the zero register.
    ///
    /// Also referred to as `x0`.
    pub(super) zero: ZeroRegister<T>,

    /// Return address register.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x1`.
    pub(super) ra: T,

    /// Stack pointer register
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x2`.
    pub(super) sp: T,

    /// Global pointer register
    ///
    /// Also referred to as `x2`.
    pub(super) gp: T,

    /// Thread pointer register
    ///
    /// Also referred to as `x2`.
    pub(super) tp: T,

    /// Temporary register 0.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x5`.
    pub(super) t0: T,

    /// Temporary register 1.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x6`.
    pub(super) t1: T,

    /// Temporary register 2.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x7`.
    pub(super) t2: T,

    /// Saved register 0.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x8`.
    pub(super) s0: T,

    /// Saved register 1.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x9`.
    pub(super) s1: T,

    /// Function argument register 0.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x10`.
    pub(super) a0: T,

    /// Function argument register 1.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x11`.
    pub(super) a1: T,

    /// Function argument register 2.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x12`.
    pub(super) a2: T,

    /// Function argument register 3.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x13`.
    pub(super) a3: T,

    /// Function argument register 4.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x14`.
    pub(super) a4: T,

    /// Function argument register 5.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x15`.
    pub(super) a5: T,

    /// Function argument register 6.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x16`.
    pub(super) a6: T,

    /// Function argument register 7.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x17`.
    pub(super) a7: T,

    /// Saved register 2.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x18`.
    pub(super) s2: T,

    /// Saved register 3.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x19`.
    pub(super) s3: T,

    /// Saved register 4.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x20`.
    pub(super) s4: T,

    /// Saved register 5.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x21`.
    pub(super) s5: T,

    /// Saved register 6.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x22`.
    pub(super) s6: T,

    /// Saved register 7.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x23`.
    pub(super) s7: T,

    /// Saved register 8.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x24`.
    pub(super) s8: T,

    /// Saved register 9.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x25`.
    pub(super) s9: T,

    /// Saved register 10.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x26`.
    pub(super) s10: T,

    /// Saved register 11.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x27`.
    pub(super) s11: T,

    /// Temporary register 3.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x28`.
    pub(super) t3: T,

    /// Temporary register 4.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x29`.
    pub(super) t4: T,

    /// Temporary register 5.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x30`.
    pub(super) t5: T,

    /// Temporary register 6.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x31`.
    pub(super) t6: T,
}

impl<T> core::fmt::Debug for Registers<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Registers");
        for i in 0..32 {
            let fmt_r = format!("{:?}", &self[i]);
            if fmt_r != "0" {
                debug_struct.field(&format!("{:?}", Register::from(i)).to_lowercase(), &self[i]);
            }
        }
        debug_struct.finish()
    }
}

/// A struct encapsulated the behaviour of the zero register:
/// - always yields zero when read
/// - can be assigned to but this effectively discards the value
#[derive(Default)]
pub(super) struct ZeroRegister<T> {
    /// This value will always be set to zero. Shared references are given to
    /// this value.
    zero: T,
    /// Since the zero register is hard wired to zero and should always yeild
    /// zero when read, we don't want to give out a mutable reference to it.
    /// Therefore we give out a mutable reference to `_zero`, and each time we
    /// ask for a mutable reference we reset this value to zero.
    ///
    /// Note: I'm not sure if there is a better way to do this currently.
    _zero: T,
}

impl<T> Debug for ZeroRegister<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0")
    }
}

impl<T> PartialEq for ZeroRegister<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for ZeroRegister<T> {}

impl<T> Deref for ZeroRegister<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.zero
    }
}

impl<T> DerefMut for ZeroRegister<T>
where
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self._zero = T::default();
        &mut self._zero
    }
}

/// A single processor register.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub(super) enum Register {
    /// The zero register.
    ///
    /// This register will always yield zero. Setting the destination register
    /// of a specific instruction will discard the result.
    ///
    /// In particular instructions that usually trigger read read side effects
    /// should not be triggered if their destination register is the zero register.
    ///
    /// Also referred to as `x0`.
    ZERO = 0,

    /// Return address register.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x1`.
    RA = 1,

    /// Stack pointer register
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x2`.
    SP = 2,

    /// Global pointer register
    ///
    /// Also referred to as `x2`.
    GP = 3,

    /// Thread pointer register
    ///
    /// Also referred to as `x2`.
    TP = 4,

    /// Temporary register 0.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x5`.
    T0 = 5,

    /// Temporary register 1.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x6`.
    T1 = 6,

    /// Temporary register 2.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x7`.
    T2 = 7,

    /// Saved register 0 / frame pointer.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x8`.
    S0 = 8,

    /// Saved register 1.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x9`.
    S1 = 9,

    /// Function argument register 0 / return value 0
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x10`.
    A0 = 10,

    /// Function argument register 1 / return value 1.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x11`.
    A1 = 11,

    /// Function argument register 2.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x12`.
    A2 = 12,

    /// Function argument register 3.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x13`.
    A3 = 13,

    /// Function argument register 4.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x14`.
    A4 = 14,

    /// Function argument register 5.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x15`.
    A5 = 15,

    /// Function argument register 6.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x16`.
    A6 = 16,

    /// Function argument register 7.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x17`.
    A7 = 17,

    /// Saved register 2.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x18`.
    S2 = 18,

    /// Saved register 3.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x19`.
    S3 = 19,

    /// Saved register 4.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x20`.
    S4 = 20,

    /// Saved register 5.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x21`.
    S5 = 21,

    /// Saved register 6.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x22`.
    S6 = 22,

    /// Saved register 7.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x23`.
    S7 = 23,

    /// Saved register 8.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x24`.
    S8 = 24,

    /// Saved register 9.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x25`.
    S9 = 25,

    /// Saved register 10.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x26`.
    S10 = 26,

    /// Saved register 11.
    ///
    /// Usually the value will be set by the callee.
    ///
    /// Also referred to as `x27`.
    S11 = 27,

    /// Temporary register 3.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x28`.
    T3 = 28,

    /// Temporary register 4.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x29`.
    T4 = 29,

    /// Temporary register 5.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x30`.
    T5 = 30,

    /// Temporary register 6.
    ///
    /// Usually the value will be set by the caller.
    ///
    /// Also referred to as `x31`.
    T6 = 31,
}

impl Register {
    /// Frame pointer register.
    ///
    /// Actually an alias to register `S0`.
    pub(super) const FP: Self = Self::S0;
    /// Converts the [u8] into a [Register].
    ///
    /// The [u8] will be masked to ensure it always returns a valid register.
    ///
    /// # Example
    ///
    /// ```ignored
    /// assert_eq!(Register::const_from(0), Register::ZERO);
    /// assert_eq!(Register::const_from(32), Register::ZERO);
    /// ```
    pub(crate) const fn const_from(value: u8) -> Register {
        match value & 0b_0001_1111 {
            0 => Register::ZERO,
            1 => Register::RA,
            2 => Register::SP,
            3 => Register::GP,
            4 => Register::TP,
            5 => Register::T0,
            6 => Register::T1,
            7 => Register::T2,
            8 => Register::S0,
            9 => Register::S1,
            10 => Register::A0,
            11 => Register::A1,
            12 => Register::A2,
            13 => Register::A3,
            14 => Register::A4,
            15 => Register::A5,
            16 => Register::A6,
            17 => Register::A7,
            18 => Register::S2,
            19 => Register::S3,
            20 => Register::S4,
            21 => Register::S5,
            22 => Register::S6,
            23 => Register::S7,
            24 => Register::S8,
            25 => Register::S9,
            26 => Register::S10,
            27 => Register::S11,
            28 => Register::T3,
            29 => Register::T4,
            30 => Register::T5,
            31 => Register::T6,
            _ => panic!("Unreachable: Already masked the value"),
        }
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Register {
        Self::const_from(value)
    }
}

impl<T> std::ops::Index<u8> for Registers<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.zero,
            1 => &self.ra,
            2 => &self.sp,
            3 => &self.gp,
            4 => &self.tp,
            5 => &self.t0,
            6 => &self.t1,
            7 => &self.t2,
            8 => &self.s0,
            9 => &self.s1,
            10 => &self.a0,
            11 => &self.a1,
            12 => &self.a2,
            13 => &self.a3,
            14 => &self.a4,
            15 => &self.a5,
            16 => &self.a6,
            17 => &self.a7,
            18 => &self.s2,
            19 => &self.s3,
            20 => &self.s4,
            21 => &self.s5,
            22 => &self.s6,
            23 => &self.s7,
            24 => &self.s8,
            25 => &self.s9,
            26 => &self.s10,
            27 => &self.s11,
            28 => &self.t3,
            29 => &self.t4,
            30 => &self.t5,
            31 => &self.t6,
            _ => panic!("Out of bounds"),
        }
    }
}

impl<T> std::ops::Index<Register> for Registers<T> {
    type Output = T;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::ZERO => &self.zero,
            Register::RA => &self.ra,
            Register::SP => &self.sp,
            Register::GP => &self.gp,
            Register::TP => &self.tp,
            Register::T0 => &self.t0,
            Register::T1 => &self.t1,
            Register::T2 => &self.t2,
            Register::S0 => &self.s0,
            Register::S1 => &self.s1,
            Register::A0 => &self.a0,
            Register::A1 => &self.a1,
            Register::A2 => &self.a2,
            Register::A3 => &self.a3,
            Register::A4 => &self.a4,
            Register::A5 => &self.a5,
            Register::A6 => &self.a6,
            Register::A7 => &self.a7,
            Register::S2 => &self.s2,
            Register::S3 => &self.s3,
            Register::S4 => &self.s4,
            Register::S5 => &self.s5,
            Register::S6 => &self.s6,
            Register::S7 => &self.s7,
            Register::S8 => &self.s8,
            Register::S9 => &self.s9,
            Register::S10 => &self.s10,
            Register::S11 => &self.s11,
            Register::T3 => &self.t3,
            Register::T4 => &self.t4,
            Register::T5 => &self.t5,
            Register::T6 => &self.t6,
        }
    }
}

impl<T> std::ops::IndexMut<u8> for Registers<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.zero,
            1 => &mut self.ra,
            2 => &mut self.sp,
            3 => &mut self.gp,
            4 => &mut self.tp,
            5 => &mut self.t0,
            6 => &mut self.t1,
            7 => &mut self.t2,
            8 => &mut self.s0,
            9 => &mut self.s1,
            10 => &mut self.a0,
            11 => &mut self.a1,
            12 => &mut self.a2,
            13 => &mut self.a3,
            14 => &mut self.a4,
            15 => &mut self.a5,
            16 => &mut self.a6,
            17 => &mut self.a7,
            18 => &mut self.s2,
            19 => &mut self.s3,
            20 => &mut self.s4,
            21 => &mut self.s5,
            22 => &mut self.s6,
            23 => &mut self.s7,
            24 => &mut self.s8,
            25 => &mut self.s9,
            26 => &mut self.s10,
            27 => &mut self.s11,
            28 => &mut self.t3,
            29 => &mut self.t4,
            30 => &mut self.t5,
            31 => &mut self.t6,
            _ => panic!("Out of bounds"),
        }
    }
}

impl<T> std::ops::IndexMut<Register> for Registers<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::ZERO => &mut self.zero,
            Register::RA => &mut self.ra,
            Register::SP => &mut self.sp,
            Register::GP => &mut self.gp,
            Register::TP => &mut self.tp,
            Register::T0 => &mut self.t0,
            Register::T1 => &mut self.t1,
            Register::T2 => &mut self.t2,
            Register::S0 => &mut self.s0,
            Register::S1 => &mut self.s1,
            Register::A0 => &mut self.a0,
            Register::A1 => &mut self.a1,
            Register::A2 => &mut self.a2,
            Register::A3 => &mut self.a3,
            Register::A4 => &mut self.a4,
            Register::A5 => &mut self.a5,
            Register::A6 => &mut self.a6,
            Register::A7 => &mut self.a7,
            Register::S2 => &mut self.s2,
            Register::S3 => &mut self.s3,
            Register::S4 => &mut self.s4,
            Register::S5 => &mut self.s5,
            Register::S6 => &mut self.s6,
            Register::S7 => &mut self.s7,
            Register::S8 => &mut self.s8,
            Register::S9 => &mut self.s9,
            Register::S10 => &mut self.s10,
            Register::S11 => &mut self.s11,
            Register::T3 => &mut self.t3,
            Register::T4 => &mut self.t4,
            Register::T5 => &mut self.t5,
            Register::T6 => &mut self.t6,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn debug_formatting_registers_ignores_zeros() {
        let mut regs = Registers::default();
        regs.sp = 32;
        regs.t0 = 33;
        assert_eq!(format!("{:?}", regs), "Registers { sp: 32, t0: 33 }");
    }

    #[test]
    fn from_u8() {
        assert_eq!(Register::from(31), Register::T6);
        assert_eq!(Register::from(32), Register::ZERO);
    }

    #[test]
    fn register_zero_deref() {
        let register_zero = ZeroRegister::<i32>::default();
        assert_eq!(*register_zero, 0);
    }

    #[test]
    fn register_zero_deref_mut() {
        let mut register_zero = ZeroRegister::<i32>::default();
        *register_zero = 23;
        assert_eq!(*register_zero, 0);
    }

    #[test]
    fn register_zero_deref_mut_twice() {
        let mut register_zero = ZeroRegister::<i32>::default();
        *register_zero = 23;
        assert_eq!(register_zero.deref_mut(), &mut 0);
    }

    #[test]
    fn registers_zero_register() {
        let registers = Registers::<i64>::default();
        assert_eq!(registers[Register::ZERO], 0);
    }

    #[test]
    fn registers_zero_register_mut() {
        let mut registers = Registers::default();
        registers[Register::ZERO] = 42;
        assert_eq!(registers[Register::ZERO], 0);
    }

    #[test]
    fn index_u8() {
        let mut registers = Registers::default();
        for i in 0..32 {
            registers[i] = i;
            assert_eq!(i, registers[i]);
        }
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds_mut() {
        let mut registers = Registers::default();
        registers[32] = 32;
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let registers = Registers::<i128>::default();
        let _ = registers[32] == 0;
    }
}
