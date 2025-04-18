//! Code for managing HPRLAR12 (*Hyp Protection Region Limit Address Register 12*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR12 (*Hyp Protection Region Limit Address Register 12*)
pub struct Hprlar12(pub u32);
impl SysReg for Hprlar12 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar12 {}
impl Hprlar12 {
    #[inline]
    /// Reads HPRLAR12 (*Hyp Protection Region Limit Address Register 12*)
    pub fn read() -> Hprlar12 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar12 {}
impl Hprlar12 {
    #[inline]
    /// Writes HPRLAR12 (*Hyp Protection Region Limit Address Register 12*)
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
