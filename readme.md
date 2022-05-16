# EMBASSY FOR ESP32C3 #
# Table of Contents
1. [About](#About)
2. [Quick Start](#Quick_Start)
3. [Building + Flashing](#<Building-+-Flashing>)
4. [Testing](#Testing)

## About
A quick and messy port of the Embassy Rust framework  for RISCV esp32c3 monitors
Due to how interrupts are implemented in the esp32c3, this implementation does NOT easily port over to other RISCV microprocessors. It makes use of the trap entry point implemented in [esp32c3-hal](https://github.com/esp-rs/esp-hal/tree/main/esp32c3-hal) to set up interrupt handlers.

## Quick_Start

### Important
You need a nightly compiler to run the code in this repository. i.e. `rustup default nightly`


Run `rustup target add riscv32imc-unknown-none-elf` to add the ESP32-C3's ISA as a target

This project has been tested runing on `rustc 1.62.0-nightly (60e50fc1c 2022-04-04)`. 

If you encounter an error related to rust-src not being found, you need to add it. This is the source code of the Rust standard library, it is necessary to build the core Rust library. The error will tell you to install 
`rustup component add rust-src --toolchain nightly-<your_architecture>`

 Follow the compiler's instructions in the error :^ )

### flashing

This project uses `cargo espflash` to flash an ESP32C3 microcontroller, you can install it by running `cargo install cargo-espflash`
espflash should detect the proper baud-rate and device, in case it does not, please refer to the documentation of `cargo-espflash`
### Building + Flashing
Building and flashing is done by going into the desired example directory, e.g `cd examples/hello_world` and then running `cargo espflash --monitor` . This should also provide a monitoring output to show written results

## Testing

A test "suite" for embassy's time driver is also included in this project, it makes SYSTIMER interrupts can be properly scheduled and triggered concurrently. Refer to the flashing section to run the tests. There is also testing for the `WFI` instruction's functionality inside of a critical section, as well as a simple test to see if software interrupts can be triggered on the ESP32-C3

## Implementation
This implementation currently only contains embassy's time_driver. No other parts of the HAL are implemented :)