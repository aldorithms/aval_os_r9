#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::{Identify, Result};



#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) 
    -> Status 
    {
        uefi_services::init(&mut system_table)
            .unwrap();

        let boot_services 
            = system_table
                .boot_services();
        
        print_image_path(boot_services)
            .unwrap();
        
        info!("Hello world!");

        system_table
            .boot_services()
            .stall(10_000_000); // Pauses for 10 seconds

        loop {}

        //Status::SUCCESS
    }

    fn print_image_path(boot_services: &BootServices) 
        -> Result 
        {
            // Open the 'LoadedImage' protocol
            let loaded_image 
                = boot_services
                    .open_protocol_exclusive::<LoadedImage>
                    (
                        boot_services
                            .image_handle()
                    )?;
            
            // Open the 'DevicePathToText' protocol
            let device_path_to_text_handle 
                = *boot_services
                    .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
                    .first()
                    .expect("DevicePathToText is missing");
            
            // Open the 'DevicePathToText' protocol
            let device_path_to_text 
                = boot_services
                    .open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle,)?;
    
            // Get the image path
            let image_device_path 
                = loaded_image.file_path()
                    .expect("File path is not set");

            // Convert the image path to text
            let image_device_path_text 
                = device_path_to_text
                    .convert_device_path_to_text
                    (
                        boot_services, 
                        image_device_path, 
                        DisplayOnly(true), 
                        AllowShortcuts(false), 
                    )
                    .expect("convert_device_path_to_text failed");
    
            info!("Image path: {}", &*image_device_path_text);
            Ok(())
        }