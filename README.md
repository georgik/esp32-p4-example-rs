# ESP32-P4 Test

How to test:

```
cargo install espflash --git https://github.com/esp-rs/espflash.git --force
rustup target add --toolchain nightly riscv32imafc-unknown-none-elf
cargo run --release
```

## Simulation with Wokwi

```
cargo install cargo-espflash --git https://github.com/esp-rs/espflash.git --force
cargo espflash save-image --chip esp32p4 --release --merge --skip-padding p4.img
```