#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod batch;
mod lang_item;
mod stack_trace;
mod sync;
mod syscall;
mod trap;
mod ophelia_log;

#[macro_use]
extern crate log;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    info!("Initializing Kernel");

    debug!("Clear bss");
    clear_bss();

    if init() != 0 {
        panic!("[kernel]: Initialized Failure!!!")
    }

    println!("[kernel]: start running application...");
    batch::run_next_app();
}

fn init() -> i32 {
    debug!("Initializing Trap");
    trap::init();

    debug!("Initializing Batch");
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
