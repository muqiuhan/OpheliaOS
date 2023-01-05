#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod batch;
mod lang_item;
mod sync;
mod trap;
mod syscall;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    println!("[KERNEL] initializting...");
    
    if init() != 0 {
        panic!("[KERNEL] initializtion failed!!!")
    }

    println!("[KERNEL] start running application...");
    batch::run_next_app();
}

/// init kernel
fn init() -> i32 {
    println!("[KERNEL] initializing bss...");
    clear_bss();

    println!("[KERNEL] initializting trap...");
    trap::init();

    println!("[KERNEL] initializing batch...");
    batch::init();
    
    0
}

/// Before using any global variables assigned to the .bss section
/// we need to make sure the .bss section is cleared.
fn clear_bss() {
    // Try to find the global symbols sbss and ebss from elsewhere,
    // which are given by the linker script linker.ld:
    extern "C" {
        fn sbss();
        fn ebss();
    }

    // Traversing the address range and clearing byte by byte:
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
