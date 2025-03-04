# Support for Arm Cortex-R (AArch32) and Arm Cortex-A (AArch32)

This crate provides access to CPU registers and common peripherals for:

* Armv7-R Processors, like the Arm Cortex-R5
* Armv8-R AArch32 Processors, like the Arm Cortex-R52
* Armv7-A Processors, like the Arm Cortex-A5
* Armv8-A AArch32 Processors, like the Arm Cortex-A53 running in 32-bit mode

It does not support processors running in AArch64 mode - A64 machine code uses
different instructions for reading/writing system registers.

This crate contains:

* Raw register access to many Armv7-R and Armv8-R AArch32 system registers
* A driver for the PMSAv7 Memory Protection Unit (MPU)
* A driver for the PMSAv8-R Memory Protection Unit (MPU)
* A driver for the Arm Generic Timer

If you need a driver for the Arm Generic Interrupt Controller, see
<https://github.com/google/arm-gic>.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.82.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## Licence

Copyright (c) Ferrous Systems, 2025

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
