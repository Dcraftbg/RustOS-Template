#![no_std]
#![no_main]
mod panic;
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    loop { }
}
