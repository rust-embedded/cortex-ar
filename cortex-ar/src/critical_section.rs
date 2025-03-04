//! Code that implements the `critical-section` traits on Cortex-R or Cortex-A when only a single
//! core is used.
//!
//! Only valid if you have a single core.

use core::sync::atomic;

struct SingleCoreCriticalSection;
critical_section::set_impl!(SingleCoreCriticalSection);

unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // the i bit means "masked"
        let was_active = !crate::register::Cpsr::read().i();
        crate::interrupt::disable();
        atomic::compiler_fence(atomic::Ordering::SeqCst);
        was_active
    }

    unsafe fn release(was_active: critical_section::RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            atomic::compiler_fence(atomic::Ordering::SeqCst);
            // Safety: This is OK because we're releasing a lock that was
            // entered with interrupts enabled
            unsafe {
                crate::interrupt::enable();
            }
        }
    }
}
