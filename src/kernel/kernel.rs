// https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/02-booting/uefi/index.md

#![no_std]
#![no_main]

use log::info;
use uefi::prelude::*;

#[entry]
fn efi_main(
    _image_handle: Handle,
    mut system_table: SystemTable<Boot>
) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    info!("Hello World!");

    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}