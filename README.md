# esp32-c3 runtime benchmarks
The goal of this crate is highlighting the gains of hooking the interrupt vector table directly over using the generic HAL handler.

## Results
The amount of cycles taken between the triggering of an interrupt and handler entry was measured to 594 cycles for the direct option, and 3942 cycles for the default HAL option, an 85% decrease in latency.

## Run it yourself
`cargo-embed` is required for flashing. We use the imc target.
```shell
rustup target add riscv32imc-unknown-none-elf
cargo install cargo-embed
```

``` shell
cargo embed --example benchmark-hal --release
```
To run the HAL driven example.

``` shell
cargo embed --example benchmark-direct --features direct --release
```
To run the direct example.
