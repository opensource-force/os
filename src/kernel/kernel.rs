#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::alloc::Layout;
use alloc::vec::Vec;
use log::info;

use uefi::prelude::*;
use uefi_services::{ init, println };

#[entry]
fn efi_main(
    _image_handle: Handle,
    mut sys_table: SystemTable<Boot>
) -> Status {
    init(&mut sys_table).unwrap();

    let mut v = Vec::new();
    v.push("Hello");
    v.push("World");
    
    println!("alloc test = {:?}", v);

    sys_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("OOM")
}