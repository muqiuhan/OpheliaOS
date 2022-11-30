#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_item;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    if main() != 0 {
        panic!("Kernel panicked!")
    }
    panic!("Shutdown machine!")
}

fn main() -> i32 {
    println!("OpheliaOS v0.0.1");
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
