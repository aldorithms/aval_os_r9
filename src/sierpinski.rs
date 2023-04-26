use uefi::{ table::boot::BootServices, Result, Handle, proto::{ console::gop::GraphicsOutput, rng::Rng } };

mod buffer;
mod point;
mod get_random_usize;


pub fn draw_sierpinski(bt: &BootServices) -> Result <()> {
    // Open graphics output protocol.
    let gop_handle: Handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop: uefi::table::boot::ScopedProtocol<GraphicsOutput> = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;
            
    // Open random number generator protocol.
    let rng_handle: Handle = bt.get_handle_for_protocol::<Rng>()?;
    let mut rng: uefi::table::boot::ScopedProtocol<Rng> = bt.open_protocol_exclusive::<Rng>(rng_handle)?;
            
    // Create a buffer to draw into.
    let (width, height) = gop.current_mode_info().resolution();
    
    let mut buffer: buffer::Buffer = buffer::Buffer::new(width, height);
            
    // Initialize the buffer with a simple gradient background.
    for y in 0..height {
        let r: f32 = ((y as f32) / ((height - 1) as f32)) * 255.0;
        for x in 0..width {
            let g: f32 = ((x as f32) / ((width - 1) as f32)) * 255.0;
            let pixel: &mut uefi::proto::console::gop::BltPixel = buffer.pixel(x, y).unwrap();
            pixel.red = r as u8;
            pixel.green= g as u8;
            pixel.blue = 255;
        }
    }
        
    let size: point::Point = point::Point::new(width as f32, height as f32);
            
    // Define the vertices of a big triangle.
    let border: f32 = 20.0;
    
    let triangle: [point::Point; 3] = [ point::Point::new(size.x / 2.0, border), point::Point::new(border, size.y - border), point::Point::new(size.x - border, size.y - border)];
            
    // `p` is the point to draw. Start at the center of the triangle.
    let mut p: point::Point = point::Point::new(size.x / 2.0, size.y / 2.0);
            
    // Loop forever, drawing the frame after each new point is changed.
    loop {
        // Choose one of the triangle's vertices at random.
        let v: point::Point = triangle[get_random_usize::get_random_usize(&mut rng) % 3];
            
        // Move `p` halfway to the chosen vertex.
        p.x  = (p.x + v.x) * 0.5;
        p.y = (p.y + v.y) * 0.5;
            
        // Set `p` to black.
        let pixel: &mut uefi::proto::console::gop::BltPixel = buffer.pixel(p.x as usize, p.y as usize).unwrap();
            
        // Set `p` to green.
        pixel.red = 0;
        pixel.green = 100;
        pixel.blue = 0;
            
        // Draw the buffer to the screen.
        buffer.blit(&mut gop)?;
    }
} //end of draw_sierpinski