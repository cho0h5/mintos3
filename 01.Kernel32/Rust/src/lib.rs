#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panik(_info: &PanicInfo) -> ! {
    kPrintString(0, 0, "[{(!PANIC!)}]".as_ptr());
    loop {}
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
pub extern "C" fn kPrintString(x: usize, y: usize, message: *const u8) {
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
    #[inline(never)]
    fn ptr_set_value(ptr: *mut u32, value: u32) {
        unsafe { *ptr = value }
    }

    for i in 0x100000..0x600000 {
        let pdw_current_address = i as *mut u32;

        ptr_set_value(pdw_current_address, 0x00);
        unsafe {
            if *pdw_current_address != 0x00 {
                return 0;
            }
        }
    }
    1
}

#[no_mangle]
pub extern "C" fn kIsMemoryEnough() -> u8 {
    #[inline(never)]
    fn ptr_set_value(ptr: *mut u32, value: u32) {
        unsafe { *ptr = value }
    }

    for i in (0x100000..0x4000000).step_by(0x100000 / 4) {
        let pdw_current_address = i as *mut u32;

        unsafe {
            ptr_set_value(pdw_current_address, 0x12345678);
            if *pdw_current_address != 0x12345678 {
                return 0;
            }
        }
    }
    1
}

fn k_read_cpuid(eax_input: u32) -> (u32, u32, u32, u32) {
    use core::arch::asm;

    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm!("cpuid", in("eax") eax_input, lateout("eax") eax, lateout("ebx") ebx, lateout("ecx") ecx, lateout("edx") edx);
    }
    (eax, ebx, ecx, edx)
}

#[no_mangle]
pub extern "C" fn print_cpu_manufacturer() {
    let vendor: [u8; 13] = [0; 13];

    let (_, ebx, ecx, edx) = k_read_cpuid(0x00);
    let ptr_vendor = vendor.as_ptr() as *mut u32;
    unsafe {
        *ptr_vendor.add(0) = ebx;
        *ptr_vendor.add(1) = edx;
        *ptr_vendor.add(2) = ecx;
    }
    kPrintString(45, 7, vendor.as_ptr());
}

#[no_mangle]
pub extern "C" fn is_support_64() -> u8 {
    let (_, _, _, edx) = k_read_cpuid(0x80000001);

    (edx & (1 << 29) != 0) as u8
}
