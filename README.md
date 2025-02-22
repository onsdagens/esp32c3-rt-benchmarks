# esp32-c3 runtime benchmarks
The goal of this crate is highlighting the gains of hooking the interrupt vector table directly over using the generic HAL handler.

## Results
The amount of cycles taken between the triggering of an interrupt and handler entry was measured to 473 cycles for the direct option, and 3492 cycles for the default HAL option, an 86% decrease in latency.

## Run it yourself
**NOTE:** The benchmarks are intended for the ESP32-C3 microcontroller, and tested using the ESP32-C3-DevKit-RUST-1. The idea is compatible with the other Espressif RISC-V MCUs, but the benchmarks themselves may require some adjustments to compile and flash.


`cargo-espflash and espflash` are required for flashing. We use the imc target.
```shell
rustup target add riscv32imc-unknown-none-elf
cargo install cargo-espflash espflash
```

``` shell
cargo run --example benchmark-hal --release
```
To run the HAL driven example.

``` shell
cargo run --example benchmark-direct --features direct --release
```
To run the direct example.
