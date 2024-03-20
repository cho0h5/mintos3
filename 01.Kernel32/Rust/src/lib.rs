#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panik(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn gpio_put_explicit(pin: i32, on_off: bool) -> i32;
    fn sleep_ms(time: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    (a + b) * 2
}
