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
    let _ = ecall0();
    return 0;
}

const ECALL_EXIT: usize = 97;

#[link_name = "__ecall0"]
extern "system" fn ecall0(
    num: usize,
) -> usize;
