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

    // pause 10s
    //sys_table.boot_services().stall(10_000_000);
    //Status::SUCCESS

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("OOM")
}