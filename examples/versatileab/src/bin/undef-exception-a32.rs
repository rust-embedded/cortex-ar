//! Example triggering a undef exception.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use semihosting::println;

// pull in our start-up code
use versatileab::rt::{entry, exception};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    println!("Hello, this is a undef exception example");

    unsafe {
        // trigger an Undefined exception, from A32 (Arm) mode
        udf_from_a32();
    }

    println!("Recovered from fault OK!");

    semihosting::process::exit(0);
}

// These functions are written in assembly
extern "C" {
    fn udf_from_a32();
}

core::arch::global_asm!(
    r#"
    // fn udf_from_a32();
    .arm
    .global udf_from_a32
    .type udf_from_a32, %function
    udf_from_a32:
        udf     #0
        bx      lr
    .size udf_from_a32, . - udf_from_a32
"#
);

#[exception(PrefetchAbort)]
fn prefetch_abort_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(Undefined)]
unsafe fn undefined_handler(addr: usize) -> usize {
    println!("undefined abort occurred");

    if addr == udf_from_a32 as usize {
        println!("caught udf_from_a32");
    } else {
        println!(
            "Bad fault address {:08x} is not {:08x}",
            addr, udf_from_a32 as usize
        );
    }

    match COUNTER.fetch_add(1, Ordering::Relaxed) {
        0 => {
            // first time, huh?
            // go back and do it again
            println!("Doing it again");
            addr
        }
        1 => {
            // second time, huh?
            // go back but skip the instruction
            println!("Skipping instruction");
            addr + 4
        }
        _ => {
            // we've faulted thrice - time to quit
            panic!("_undefined_handler called too often");
        }
    }
}

#[exception(DataAbort)]
fn data_abort_handler(_addr: usize) -> ! {
    panic!("unexpected data abort exception");
}
