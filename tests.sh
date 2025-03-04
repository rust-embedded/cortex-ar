#!/bin/bash

# Runs a series of sample programs in QEMU and checks that the standard output
# is as expected.

rustup target add armv7r-none-eabi
rustup target add armv7r-none-eabihf
rustup target add armv7a-none-eabi

FAILURE=0

fail() {
    echo "***************************************************"
    echo "test.sh MISMATCH: Binary $1 for target $2 mismatched"
    echo "***************************************************"
    FAILURE=1
}

mkdir -p ./target

versatile_ab_cargo="--manifest-path examples/versatileab/Cargo.toml"

# armv7r-none-eabi tests
for binary in hello registers svc; do
    cargo run ${versatile_ab_cargo} --target=armv7r-none-eabi --bin $binary | tee ./target/$binary-armv7r-none-eabi.out
    diff ./examples/versatileab/reference/$binary-armv7r-none-eabi.out ./target/$binary-armv7r-none-eabi.out || fail $binary "armv7r-none-eabi"
done

# armv7r-none-eabihf tests
for binary in hello registers svc; do
    cargo run ${versatile_ab_cargo} --target=armv7r-none-eabihf --bin $binary | tee ./target/$binary-armv7r-none-eabihf.out
    diff ./examples/versatileab/reference/$binary-armv7r-none-eabihf.out ./target/$binary-armv7r-none-eabihf.out || fail $binary "armv7r-none-eabihf"
done

# armv7a-none-eabi tests
for binary in hello registers svc; do
    cargo run ${versatile_ab_cargo} --target=armv7a-none-eabi --bin $binary | tee ./target/$binary-armv7a-none-eabi.out
    diff ./examples/versatileab/reference/$binary-armv7a-none-eabi.out ./target/$binary-armv7a-none-eabi.out || fail $binary "armv7a-none-eabi"
done

# Ubuntu 24.04 supplies QEMU 8, which doesn't support the machine we have configured for this target
# # armv8r-none-eabihf tests
# for binary in hello registers svc gic; do
#     cargo +nightly run --target=armv8r-none-eabihf --bin $binary --features=gic -Zbuild-std=core | tee ./target/$binary-armv8r-none-eabihf.out
#     diff ./cortex-r-examples/reference/$binary-armv8r-none-eabihf.out ./target/$binary-armv8r-none-eabihf.out || fail $binary "armv8r-none-eabihf"
# done

if [ "$FAILURE" == "1" ]; then
    echo "***************************************************"
    echo "test.sh: Output comparison failed!"
    echo "***************************************************"
    exit 1
else
    echo "***************************************************"
    echo "test.sh: Everything matches :)"
    echo "***************************************************"
fi
