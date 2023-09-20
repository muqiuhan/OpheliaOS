#![no_std]
#![no_main]

use core::arch::global_asm;
mod basic_components;
mod boot;

global_asm!(include_str!("./asm/boot.S"));

#[no_mangle]
pub fn ophelia_os_entry() -> ! {
    boot::clear_bss();
    loop {}
}
