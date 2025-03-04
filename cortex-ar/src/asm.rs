//! Simple assembly routines

/// Emit an DSB instruction
#[inline]
pub fn dsb() {
    unsafe {
        core::arch::asm!("dsb");
    }
}

/// Emit an ISB instruction
#[inline]
pub fn isb() {
    unsafe {
        core::arch::asm!("isb");
    }
}

/// Emit an NOP instruction
#[inline]
pub fn nop() {
    unsafe {
        core::arch::asm!("nop");
    }
}

/// Emit an WFI instruction
#[inline]
pub fn wfi() {
    unsafe {
        core::arch::asm!("wfi");
    }
}

/// Emit an WFE instruction
#[inline]
pub fn wfe() {
    unsafe {
        core::arch::asm!("wfe");
    }
}

/// Emit an SEV instruction
#[inline]
pub fn sev() {
    unsafe {
        core::arch::asm!("sev");
    }
}
