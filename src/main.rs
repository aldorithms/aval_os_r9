#![no_main]
#![no_std]

extern crate alloc;

use log::info;

//use uefi::prelude::*;
use uefi::
{ 
    table::boot::SearchType, prelude::*, Identify, Result, proto::
    {
        console::gop::GraphicsOutput,
        loaded_image::LoadedImage,
        rng::Rng,
        device_path::text::
        {
            AllowShortcuts, 
            DevicePathToText, 
            DisplayOnly
        },
    },
};



mod buffer;
mod getranusize;
mod point;
//mod sierpinski;

#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) // Entry point of the OS
    -> Status
    {
        uefi_services::init(&mut system_table) // Initialize the UEFI services
            .unwrap(); // Initialize the logger

        let boot_services // Get the boot services
            = system_table // Get the boot services
                .boot_services(); // Get the boot services
        
        print_image_path(boot_services) // Print the image path
            .unwrap(); // Unwrap the result
        
        info!("Hello world!"); // Print a message to the console

        system_table
            .boot_services() // Get the boot services
            .stall(10_000_000); // Pauses for 10 seconds
        
        draw_sierpinski(boot_services)
            .unwrap();

        loop {}

        // Status::SUCCESS
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
                .locate_handle_buffer(SearchType::ByProtocol( &DevicePathToText::GUID))?
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

fn draw_sierpinski(bt: &BootServices) 
    -> Result 
    {
        // Open graphics output protocol.
        let gop_handle 
            = bt
                .get_handle_for_protocol::<GraphicsOutput>()?;
    
        let mut gop 
            = bt
                .open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;
            
        // Open random number generator protocol.
        let rng_handle 
            = bt
                .get_handle_for_protocol::<Rng>()?;
    
        let mut rng 
            = bt
                .open_protocol_exclusive::<Rng>(rng_handle)?;
            
        // Create a buffer to draw into.
        let (width, height) 
            = gop
                .current_mode_info()
                .resolution();
    
        let mut buffer
            = buffer::Buffer::new(width, height);
            
        // Initialize the buffer with a simple gradient background.
        for y in 0..height 
        {
            let r 
                = ((y as f32) / ((height - 1) as f32)) * 255.0;
    
            for x in 0..width 
            {
                let g 
                    = ((x as f32) / ((width - 1) as f32)) * 255.0;
    
                let pixel 
                    = buffer
                        .pixel(x, y)
                        .unwrap();
    
                pixel.red 
                    = r as u8;
    
                pixel.green
                    = g as u8;
    
                pixel.blue 
                    = 255;
            }
        }
            
        let size 
            = point::Point::new(width as f32, height as f32);
            
        // Define the vertices of a big triangle.
        let border 
            = 20.0;
    
        let triangle 
            =   [
                    point::Point::new(size.x / 2.0, border),
                    point::Point::new(border, size.y - border),
                    point::Point::new(size.x - border, size.y - border),
                ];
            
        // `p` is the point to draw. Start at the center of the triangle.
        let mut p 
            = point::Point::new(size.x / 2.0, size.y / 2.0);
            
        // Loop forever, drawing the frame after each new point is changed.
        loop 
        {
            // Choose one of the triangle's vertices at random.
            let v 
                = triangle[getranusize::get_random_usize(&mut rng) % 3];
            
            // Move `p` halfway to the chosen vertex.
            p.x 
                = (p.x + v.x) * 0.5;
    
            p.y 
                = (p.y + v.y) * 0.5;
            
            // Set `p` to black.
            let pixel 
                = buffer
                    .pixel(p.x as usize, p.y as usize)
                    .unwrap();
            
            // Set `p` to green.
            pixel.red 
                = 0;
    
            pixel.green 
                = 100;
    
            pixel.blue 
                = 0;
            
            // Draw the buffer to the screen.
            buffer
                .blit(&mut gop)?;
        }
    }