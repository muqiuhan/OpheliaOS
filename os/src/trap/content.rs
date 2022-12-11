/// The physical resource content that needs to be saved when the Trap occurs:
#[repr(C)]
pub struct TrapContent {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}
