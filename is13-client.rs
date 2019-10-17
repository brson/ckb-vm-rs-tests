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
    unsafe {
        let _ = ecall0(ECALL_EXIT);
        return 0;
    }
}

const ECALL_EXIT: usize = 97;

extern "system" {
    #[link_name = "__ecall0"]
    fn ecall0(
        num: usize,
    ) -> usize;
}
