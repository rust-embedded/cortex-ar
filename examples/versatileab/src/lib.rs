//! Common code for all examples

#![no_std]

// Need this to bring in the start-up function
#[cfg(arm_profile = "a")]
use cortex_a_rt as _;
#[cfg(arm_profile = "r")]
use cortex_r_rt as _;

#[cfg(arm_architecture = "v8-r")]
compile_error!("This example/board is not compatible with the ARMv8-R architecture");

#[macro_export]
macro_rules! entry_point {
    () => {
        /// The entry-point to the Rust application.
        ///
        /// It is called by the start-up code in `cortex-m-rt`.
        #[no_mangle]
        #[cfg(arm_profile = "r")]
        pub extern "C" fn kmain() {
            main();
        }

        /// The entry-point to the Rust application.
        ///
        /// It is called by the start-up code in `cortex-a-rt`.
        #[no_mangle]
        #[cfg(arm_profile = "a")]
        pub extern "C" fn boot_core(cpu_id: u32) {
            match cpu_id {
                0 => {
                    main();
                }
                _ => panic!("unexpected CPU ID {}", cpu_id),
            }
        }
    };
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    semihosting::process::abort();
}
