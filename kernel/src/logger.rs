////////////////////////////////////////////////////////////////////////////////////
// The MIT License (MIT)							  //
// 										  //
// Copyright (c) 2022 Muqiu Han							  //
// 										  //
// Permission is hereby granted, free of charge, to any person obtaining a copy	  //
// of this software and associated documentation files (the "Software"), to deal  //
// in the Software without restriction, including without limitation the rights	  //
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell	  //
// copies of the Software, and to permit persons to whom the Software is	  //
// furnished to do so, subject to the following conditions:			  //
// 										  //
// The above copyright notice and this permission notice shall be included in all //
// copies or substantial portions of the Software.				  //
// 										  //
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR	  //
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,	  //
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE	  //
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER	  //
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,  //
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE  //
// SOFTWARE.									  //
////////////////////////////////////////////////////////////////////////////////////

use core::fmt::{self, Write};

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

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logger::log(format_args!(concat!("\x1b[32m", "[KERNEL] [INFO   ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logger::log(format_args!(concat!("\x1b[34m", "[KERNEL] [DEBUG  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! warning {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logger::log(format_args!(concat!("\x1b[93m". "[KERNEL] [WARNING]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logger::log(format_args!(concat!("\x1b[90m", "[KERNEL] [TRACE  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logger::log(format_args!(concat!("\x1b[31m", "[KERNEL] [ERROR  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}
