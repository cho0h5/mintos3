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

#[repr(C)]
struct Character {
    charactor: u8,
    attribute: u8,
}

fn c_str_len(c_str: *const u8) -> usize {
    let mut len = 0;

    unsafe {
        while *c_str.add(len) != 0 {
            len += 1;
        }
    }

    len
}

fn c_str_to_slice(c_str: *const u8) -> &'static [u8] {
    let len = c_str_len(c_str);

    unsafe { core::slice::from_raw_parts(c_str, len) }
}

#[no_mangle]
pub extern "C" fn kPrintString(x: usize, y: usize, message: *mut u8) {
    let message = c_str_to_slice(message);
    let mut pst_screen = 0xB8000 as *mut Character;

    unsafe {
        pst_screen = pst_screen.add((y * 80) + x);
    }

    for (i, c) in message.iter().enumerate() {
        unsafe {
            (*pst_screen.add(i)).charactor = *c;
        }
    }
}

#[no_mangle]
pub extern "C" fn kInitializeKernel64Area() -> u8 {
    for i in 0x100000..0x600000 {
        let pdw_current_address = i as *mut u32;

        unsafe {
            *pdw_current_address = 0x00;

            if *pdw_current_address != 0x00 {
                return 0;
            }
        }
    }
    1
}

#[no_mangle]
pub extern "C" fn kIsMemoryEnough() -> u8 {
    for i in (0x100000..0x4000000).step_by(0x100000 / 4) {
        let pdw_current_address = i as *mut u32;

        unsafe {
            *pdw_current_address = 0x12345678;
            if *pdw_current_address != 0x12345678 {
                return 0;
            }
        }
    }
    1
}
