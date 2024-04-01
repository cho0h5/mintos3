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

#[repr(C)]
struct PageTableEntryStruct {
    dw_attribute_and_lower_base_address: u32,
    dw_upper_base_address_and_exb: u32,
}

const PAGE_FLAGS_P: u32 = 0x00000001;
const PAGE_FLAGS_RW: u32 = 0x00000002;
#[allow(dead_code)]
const PAGE_FLAGS_US: u32 = 0x00000004;
#[allow(dead_code)]
const PAGE_FLAGS_PWT: u32 = 0x00000008;
#[allow(dead_code)]
const PAGE_FLAGS_PCD: u32 = 0x00000010;
#[allow(dead_code)]
const PAGE_FLAGS_A: u32 = 0x00000020;
#[allow(dead_code)]
const PAGE_FLAGS_D: u32 = 0x00000040;
const PAGE_FLAGS_PS: u32 = 0x00000080;
#[allow(dead_code)]
const PAGE_FLAGS_G: u32 = 0x00000100;
#[allow(dead_code)]
const PAGE_FLAGS_PAT: u32 = 0x00001000;
#[allow(dead_code)]
const PAGE_FLAGS_EXB: u32 = 0x80000000;
const PAGE_FLAGS_DEFAULT: u32 = PAGE_FLAGS_P | PAGE_FLAGS_RW;

const PAGE_TABLESIZE: u32 = 0x100;
const PAGE_MAXENTRYCOUNT: usize = 512;
const PAGE_DEFAULTSIZE: u32 = 0x200000;

#[inline(never)]
fn k_set_page_entry_data(
    pst_entry: *mut PageTableEntryStruct,
    dw_upper_base_address: u32,
    dw_lower_base_address: u32,
    dw_lower_flags: u32,
    dw_upper_flags: u32,
) {
    unsafe {
        (*pst_entry).dw_attribute_and_lower_base_address = dw_lower_base_address | dw_lower_flags;
        (*pst_entry).dw_upper_base_address_and_exb =
            (dw_upper_base_address & 0xff) | dw_upper_flags;
    }
}

#[no_mangle]
pub extern "C" fn k_initialize_page_tables() {
    let pst_pml4_entry = 0x100000 as *mut PageTableEntryStruct;
    k_set_page_entry_data(pst_pml4_entry, 0x00, 0x10100, PAGE_FLAGS_DEFAULT, 0);
    for i in 1..PAGE_MAXENTRYCOUNT {
        unsafe {
            k_set_page_entry_data(pst_pml4_entry.add(i), 0, 0, 0, 0);
        }
    }

    let pst_pdptentry = 0x101000 as *mut PageTableEntryStruct;
    for i in 0..64 {
        unsafe {
            k_set_page_entry_data(
                pst_pdptentry.add(i),
                0,
                0x102000 + (i as u32 * PAGE_TABLESIZE),
                PAGE_FLAGS_DEFAULT,
                0,
            );
        }
    }
    for i in 64..PAGE_MAXENTRYCOUNT {
        unsafe {
            k_set_page_entry_data(pst_pdptentry.add(i), 0, 0, 0, 0);
        }
    }

    let pst_pdentry = 0x102000 as *mut PageTableEntryStruct;
    let mut dw_mapping_address = 0;
    for i in 0..PAGE_MAXENTRYCOUNT * 64 {
        unsafe {
            k_set_page_entry_data(
                pst_pdentry.add(i),
                (i as u32 * (PAGE_DEFAULTSIZE >> 20)) >> 12,
                dw_mapping_address,
                PAGE_FLAGS_DEFAULT | PAGE_FLAGS_PS,
                0,
            )
        }
        dw_mapping_address += PAGE_DEFAULTSIZE;
    }
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
