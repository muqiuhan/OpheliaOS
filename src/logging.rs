#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println!(format_args!(concat!("\x1b[32m", "[KERNEL] [INFO   ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println!(format_args!(concat!("\x1b[34m", "[KERNEL] [DEBUG  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! warning {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println!(format_args!(concat!("\x1b[93m". "[KERNEL] [WARNING]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println!(format_args!(concat!("\x1b[90m", "[KERNEL] [TRACE  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println!(format_args!(concat!("\x1b[31m", "[KERNEL] [ERROR  ]: ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}