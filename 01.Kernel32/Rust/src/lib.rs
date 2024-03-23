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
pub extern "C" fn kInitializeKernel64Area() -> u8 {
    for i in 0x100000..0x600000 {
        let pdwCurrentAddress = i as *mut u32;

        unsafe {
            *pdwCurrentAddress = 0x00;

            if *pdwCurrentAddress != 0x00 {
                return 0;
            }
        }
    }
    1
}
