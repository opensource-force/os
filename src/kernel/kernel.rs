// https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/02-booting/uefi/index.md

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

use uefi::prelude::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! { loop {} }


#[entry]
fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    let stdout = system_table.stdout();
    
    writeln!(stdout, "Hello World!").unwrap();

    loop {}
}