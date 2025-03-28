//! critical-section example for Arm Cortex-R

#![no_std]
#![no_main]

use core::cell::RefCell;

// pull in our start-up code
use mps3_an536 as _;

use semihosting::println;

struct Data {
    value: u32
}

static GLOBAL_DATA: critical_section::Mutex<RefCell<Data>> = critical_section::Mutex::new(RefCell::new(Data { value: 100 }));

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[no_mangle]
pub extern "C" fn kmain() {
    main();
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> ! {
    let value = critical_section::with(|cs| {
        let mut data = GLOBAL_DATA.borrow_ref_mut(cs);
        data.value += 1;
        data.value
    });
    println!("Data is {}", value);
    panic!("I am an example panic");
}
