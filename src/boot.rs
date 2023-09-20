/// The `sbss` and `ebss` provide by the `lds/linker.ld`
/// Indicate respectively the statring and ending addresses of the `.bss` section
/// that needs to be cleared.
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    // Reference the location flag and convert it to usize to gt its address
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
