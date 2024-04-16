#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

extern crate alloc;

use core::{ mem, slice };
use alloc::alloc::Layout;
use alloc::vec::Vec;

use uefi::{
    prelude::*,
    table::{ cfg, boot::{
        OpenProtocolParams,
        OpenProtocolAttributes,
        MemoryDescriptor,
        MemoryType
    } },
    proto::console::gop::GraphicsOutput
};
use uefi_services::{ init, println };

#[entry]
fn efi_main(
    _image_handle: Handle,
    mut sys_table: SystemTable<Boot>
) -> Status {
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

    // locating ACPI RSDP tables with UEFI config table
    // Section 4.6: https://uefi.org/sites/default/files/resources/UEFI%20Spec%202.8B%20May%202020.pdf
    // Section 5.2.5.2: https://uefi.org/sites/default/files/resources/ACPI_6_2.pdf
    // may remove for bootloader crate
    let mut config_entries = sys_table.config_table().iter();
    let rsdp_addr = config_entries
        .find(|entry| matches!(entry.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|entry| entry.address).unwrap();

    println!("RSDP addr: {:?}", rsdp_addr);

    let boot_services = &sys_table.boot_services();
    /*
    let gop_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>()
        .expect("Failed acquiring GOP handle");
    
    // opening exclusely closes stdout
    // let mut gop = boot_services.open_protocol_exclusive::<GraphicsOutput>(handle).unwrap();
    let mut gop = unsafe {
        let gop_proto = boot_services.open_protocol::<GraphicsOutput>(
            OpenProtocolParams {
                handle: gop_handle,
                agent: boot_services.image_handle(),
                controller: None
            }, OpenProtocolAttributes::GetProtocol )
            .expect("Failed opening GOP");

        gop_proto
    };
    let gop_mode = gop.current_mode_info();
    let gop_frame_buf = gop.frame_buffer();

    println!("GOP: {:?} {:?}", gop_mode, gop_frame_buf);
    */
    
    let max_mmap_size = boot_services.memory_map_size().map_size
        + 8 * mem::size_of::<MemoryDescriptor>();
    let ptr = boot_services
        .allocate_pool(MemoryType::LOADER_DATA, max_mmap_size).unwrap();
        
    let _ = unsafe { slice::from_raw_parts_mut(ptr, max_mmap_size) };
    
    //RUNTIME_SERVICES_DATA
    let (_sys_table, _mmap) = sys_table.exit_boot_services(MemoryType::LOADER_DATA);

    // pause 10s
    //sys_table.boot_services().stall(10_000_000);
    //Status::SUCCESS

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("OOM")
}