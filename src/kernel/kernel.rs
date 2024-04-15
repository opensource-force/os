#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::alloc::Layout;
use alloc::vec::Vec;

use uefi::prelude::*;
use uefi_services::{ init, println };

#[entry]
fn efi_main(
    _image_handle: Handle,
    mut sys_table: SystemTable<Boot>
) -> Status {
    // initialize boot services
    init(&mut sys_table).unwrap();

    // test uefi allocator
    let mut v = Vec::new();
    v.push("Hello");
    v.push("World");
    
    println!("alloc test: {:?}", v);

    let firmware_vendor = sys_table.firmware_vendor();
    let firmware_rev = sys_table.firmware_revision();
    let uefi_rev_major = sys_table.uefi_revision().major();
    let uefi_rev_minor = sys_table.uefi_revision().minor();

    println!("firmware vendor: {}", firmware_vendor);
    println!("firmware revision: {}", firmware_rev);
    println!("UEFI revision: {}.{}", uefi_rev_major, uefi_rev_minor);

    // pause 10s
    //sys_table.boot_services().stall(10_000_000);
    //Status::SUCCESS

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("OOM")
}