// https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/02-booting/uefi/index.md

//#![feature(abi_efiapi)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ffi::c_void;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! { loop {} }

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image: *mut c_void,
    system_table: *const c_void,
) -> usize
{ loop {} }