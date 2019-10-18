#![allow(unused)]
#![no_std]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
fn abort() -> ! {
    loop { }
}

#[start]
#[no_mangle]
pub fn _start(argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        let _ = ecall1(ECALL_EXIT, 1);
        return foo(1);
    }
}

#[inline(never)]
pub fn foo(arg0: isize) -> isize { arg0.wrapping_add(1) }

const ECALL_EXIT: usize = 93;

extern "system" {
    #[link_name = "__ecall0"]
    fn ecall0(
        num: usize,
    ) -> usize;

    #[link_name = "__ecall1"]
    fn ecall1(
        num: usize,
        arg0: usize,
    ) -> usize;

}

