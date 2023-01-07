#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_item;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("Unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
/// We mark its function symbol main as weakly linked. In this way, at the time of the final link,
/// although an application in the lib.rs and bin directories has a main symbol, since the main
/// symbol in lib.rs is a weak link, the linker will use the main logic of the application in the
/// bin directory as main. Here we mainly carry out a certain degree of protection. If no main is
/// found in the bin directory, the compilation can pass, but an error will be reported at runtime.
fn main() -> i32 {
    panic!("Cannot find main!");
}

/// Before using any global variables assigned to the .bss section
/// we need to make sure the .bss section is cleared.
fn clear_bss() {
    // Try to find the global symbols sbss and ebss from elsewhere,
    // which are given by the linker script linker.ld:
    extern "C" {
        fn start_bss();
        fn end_bss();
    }

    // Traversing the address range and clearing byte by byte:
    (start_bss as usize..end_bss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    syscall::sys_exit(exit_code)
}
