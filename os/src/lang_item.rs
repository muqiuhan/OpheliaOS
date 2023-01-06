use crate::println;
use core::panic::PanicInfo;
use crate::stack_trace;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }

    unsafe { stack_trace::print(); }
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);

    loop {}
}
