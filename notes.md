Build with nightly:

- rustup toolchain add nightly
- rustup target add riscv32imac-unknown-none-elf --toolchain=nightly
- rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust

View assembly:

- rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust --emit=asm

Run:

- cargo run -- is13-client-rust 1