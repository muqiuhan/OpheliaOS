use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

/// Write data from an in-memory buffer to a file
/// `fd`: The file descriptor of the file to be written
/// `buf`: The starting address of the buffer in memory
/// Returns the length of a successful write.
/// __Syscall ID : 64__
pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

/// Exit the application and inform the batch system of the return value
/// `xstate`: The return value of  the application
/// This system call should not return
/// __Syscall ID : 93__
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;

    unsafe {
        asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id
        );
    }

    ret
}
