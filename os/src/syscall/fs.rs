const FD_STDOUT: usize = 1;

/// Convert the start address and length of the incoming buffer in the application into a string, and then print it.
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
	FD_STDOUT => {
	    let slice = unsafe { core::slice::from_raw_parts(buf, len) };
	    let str = core::str::from_utf8(slice).unwrap();

	    print!("{}", str);

	    len as isize
	},

	_ => {
	    panic!("Unsupported fd in sys_write")
	}
    }
}
