#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

mod sbi;
#[macro_use]
mod console;
#[macro_use]
mod logging;
mod basic_components;
mod boot;

global_asm!(include_str!("./asm/boot.S"));

#[no_mangle]
pub fn ophelia_os_entry() -> ! {
    boot::init_bss();
    boot::init_logger();
    boot::print_logo();
    sbi::shutdown(false);
}
