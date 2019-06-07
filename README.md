# Testing project for Rust Wii

`xargo build --target powerpc-unknown-eabi` to compile.

If you get an error that says 'Error loading target specification: Could not find specification for target "powerpc-unknown-eabi"', then you need to set the environment variable `$RUST_TARGET_PATH` to this project's directory.

This is most easily done by simply running `RUST_TARGET_PATH=$PWD xargo build --target powerpc-unknown-eabi` in your shell.
