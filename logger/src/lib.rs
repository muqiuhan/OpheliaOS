#![no_std]

use core::fmt::{self, Write};
use core::result::Result::Ok;

#[allow(dead_code)]
struct Log;

impl Write for Log {
    #[allow(deprecated)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            sbi_rt::legacy::console_putchar(c as usize);
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn log(args: fmt::Arguments) {
    Log.write_fmt(args).unwrap();
}

#[macro_use]
pub mod macros;
