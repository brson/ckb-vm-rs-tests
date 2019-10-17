## GCC

Building gcc:

```
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
mkdir build && cd build
../configure --prefix=$HOME/riscv-gcc --with-arch=rv32imac --with-abi=ilp32
make
```

Build with gcc:

```
~/riscv-gcc/bin/riscv32-unknown-elf-gcc is13-client.c -o is13-client-gcc
```

Emit assembly with gcc:

```
~/riscv-gcc/bin/riscv32-unknown-elf-gcc is13-client.c -S -o is13-client-gcc.s
```

Disassemble:

```
~/riscv-gcc/bin/riscv32-unknown-elf-objdump is13-client-gcc -d
```

## Rust

Installing:

```
rustup toolchain add nightly
rustup target add riscv32imac-unknown-none-elf --toolchain=nightly
```

Building:

```
rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust
```

Emit assembly:

```
rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust.s --emit=asm
```

Emit bin + assembly

```
rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust --emit=link,asm
```

Run:

```
cargo run -- is13-client-rust 1
```


## Syscalls

Build syscall trampoline:

```
~/riscv-gcc/bin/riscv32-unknown-elf-as ecall32.s -o ecall32.o
~/riscv-gcc/bin/riscv32-unknown-elf-ar -rsv libecall32.a ecall32.o
~/riscv-gcc/bin/riscv32-unknown-elf-objdump libecall32.a -d
```

Call `rustc` with syscalls:

```
rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf -o is13-client-rust --emit=link,asm -lstatic=ecall32 -Lnative=.
```
