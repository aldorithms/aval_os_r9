use log::info;

use uefi::{ table::boot::SearchType, prelude::*, Result, proto::{ loaded_image::LoadedImage, device_path::text::{ AllowShortcuts, DevicePathToText, DisplayOnly}}};


pub fn print_image_path(boot_services: &BootServices) -> Result <()> {
    // Open the 'LoadedImage' protocol
    let loaded_image: uefi::table::boot::ScopedProtocol<LoadedImage> = boot_services.open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;   
    
    // Open the 'DevicePathToText' protocol
    let device_path_to_text_handle: Handle = *boot_services.locate_handle_buffer(SearchType::ByProtocol( &DevicePathToText::GUID))?.first().expect("DevicePathToText is missing");
            
    // Open the 'DevicePathToText' protocol
    let device_path_to_text: uefi::table::boot::ScopedProtocol<DevicePathToText> = boot_services.open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle,)?;
    
    // Get the image path
    let image_device_path: &uefi::proto::device_path::DevicePath = loaded_image.file_path().expect("File path is not set");

    // Convert the image path to t: uefi::proto::device_path::text::PoolStringext
    let image_device_path_text: uefi::proto::device_path::text::PoolString = device_path_to_text.convert_device_path_to_text(boot_services, image_device_path, DisplayOnly(true), AllowShortcuts(false) ).expect("convert_device_path_to_text failed");
    
    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}