use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// For general-purpose registers, the two control flows (application control flow and kernel control flow) run at different privilege levels,
/// and the software they belong to may also be written in different programming languages,
/// although only Trap processing is performed in the Trap control flow relate code,
/// but it is still possible to directly or indirectly call many modules,
/// so it is difficult or impossible to find out which registers do not need to be saved.
/// In this case, we can only save them all.
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };

        cx.set_sp(sp);
        cx
    }
}
