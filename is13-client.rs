// Build with nightly:
//
//     rustup toolchain add nightly
//     rustup target add riscv32imac-unknown-none-elf --toolchain=nightly
//     rustc +nightly is13-client.rs --target=riscv32imac-unknown-none-elf


#![allow(unused)]
#![no_std]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { }
}

#[start]
#[no_mangle]
pub fn _start(argc: isize, argv: *const *const u8) -> isize {
    return 0;
}

