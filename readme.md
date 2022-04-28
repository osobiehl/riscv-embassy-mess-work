## EMBASSY FOR ESP32C3 ##
# Table of Contents
1. [Quick Start](#Quick_Start)
2. [Building + Flashing](#<Building-+-Flashing>)
3. [Testing](#Testing)

## About
    A quick and messy port of the Embassy Rust framework  for RISCV esp32c3 monitors
    ##NOTE## Due to how interrupts are implemented in the esp32c3, this implementation does #NOT# easily port over to other RISCV microprocessors. It makes use of the trap entry point implemented in [esp32c3-hal](https://github.com/esp-rs/esp-hal/tree/main/esp32c3-hal) to set up interrupt handlers.

## Quick_Start
### flashing
    This project uses `cargo espflash` to flash an ESP32C3 microcontroller, you can install it by running cargo install cargo-espflash`
    espflash should detect the proper baud-rate and device, in case it does not, please refer to the documentation of `cargo-espflash`
### Building + Flashing
    Building and flashing is done by going into the desired example directory, e.g `cd hello_world` and then running `cargo espflash --monitor` . This should also provide a monitoring output to show written results

## Testing
    A test "suite" for embassy's time driver is also included in this project, it makes SYSTIMER interrupts can be properly scheduled and triggered concurrently. Refer to the flashing section to run the tests

## Implementation
    This implementation currently only contains embassy's time_driver. No other parts of the HAL are implemented :)