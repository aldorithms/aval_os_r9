#![no_main]
#![no_std]

extern crate alloc;

use log::info;
use uefi::prelude::*;

mod sierpinski;
mod imgpath;

#[entry]
fn efi_main(_image_handle: Handle, mut sys_table: SystemTable<Boot>)-> Status {
    uefi_services::init(&mut sys_table).expect_success("Failed to initialize logger").unwrap();
    let boot_services: &BootServices = sys_table.expect_success.boot_services();
    imgpath::print_image_path(boot_services).unwrap();
    info!("Hello world!");
    sys_table.boot_services().stall(10_000_000);
    sierpinski::draw_sierpinski(boot_services).unwrap();
    loop {}
    // Status::SUCCESS
}