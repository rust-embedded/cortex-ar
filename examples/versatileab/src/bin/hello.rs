//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use versatileab as _;

use semihosting::println;

versatileab::entry_point!();

/// The main function of our Rust application.
#[export_name = "main"]
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am an example panic");
}
