use core::arch::asm;

use crate::sync::upsafe_cell::UPSafeCell;
use crate::trap;
use lazy_static::lazy_static;

/// Save the number of applications and their respective location information, as well as the number of applications currently executed.
/// According to the location information of the application, initialize the memory space required by the application, and load the application to execute.
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

/// The constants USER_STACK_SIZE and KERNEL_STACK_SIZE indicate that the size of the kernel stack and user stack are 8KiB respectively
/// Both types are instantiated as global variables in the .bss section of the batch os.
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; KERNEL_STACK_SIZE],
};

impl UserStack {
    /// Get the address of the top of the stack
    /// Since the stack grows downward in risc-v, it is only necessary to return the end address of the wrapped array.
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    /// Get the address of the top of the stack
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// The return value of this function is the top of the stack after the kernel stack is pushed into the Trap context,
    /// which will be used as the parameter of __restore
    pub fn push_context(&self, cx: trap::context::TrapContext) -> &'static mut trap::context::TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<trap::context::TrapContext>()) as *mut trap::context::TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl AppManager {
    
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    /// Load the binary image of the application corresponding to the parameter `app_id` to the location starting at `0x80400000` in physical memory.
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }

        println!("[kernel] Loading app_{}", app_id);

        // Normally, the CPU will think that the code segment of the program will not change, so `i-cache` is a read-only cache.
        // But here, the OS will modify the memory area that will be fetched by the CPU, which will make the `i-cache` contain inconsistent content with the memory.
        // Therefore, the OS must use the `fence.i` command to manually clear the `i-cache` to invalidate all the contents in it,
        // so as to ensure the correctness of CPU access to memory data and codes.
        asm!("fence.i");

        // clear app area
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.copy_from_slice(app_src);
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();

    unsafe {
        app_manager.load_app(current_app);
    }

    app_manager.move_to_next_app();
    drop(app_manager);

    // before this we have to drop local variables related to resource manually and release to resource
    extern "C" {
        fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(KERNEL_STACK.push_context(trap::context::TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }

    panic!("Unreachable in batch::run_current_app!");
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}
