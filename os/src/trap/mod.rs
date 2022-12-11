global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe { stvec::write(__alltraps as usize, TrapMode::Direct) }
}
