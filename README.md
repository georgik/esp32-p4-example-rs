# ESP32-P4 Test

How to test:

```
cargo install espflash --git https://github.com/SergioGasquez/espflash --branch  feat/esp32p4 --force
rustup target add --toolchain nightly riscv32imafc-unknown-none-elf
cargo run --release
```

