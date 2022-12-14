use core::panic::PanicInfo;
use stacktrace;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }

    unsafe {
        stacktrace::print();
    }
    
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    
    loop {}
}
