//! Code for managing PRLAR2 (*Protection Region Limit Address Register 2*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR2 (*Protection Region Limit Address Register 2*)
pub struct Prlar2(pub u32);
impl SysReg for Prlar2 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 9;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Prlar2 {}
impl Prlar2 {
    #[inline]
    /// Reads PRLAR2 (*Protection Region Limit Address Register 2*)
    pub fn read() -> Prlar2 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar2 {}
impl Prlar2 {
    #[inline]
    /// Writes PRLAR2 (*Protection Region Limit Address Register 2*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
