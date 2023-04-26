
use alloc::{vec, vec::Vec};
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::Result;


pub struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
} 

impl Buffer {
    /// Create a new `Buffer`.
    pub fn new(width: usize, height: usize) -> Self {
        Buffer { width, height, pixels: vec![ BltPixel::new(0, 0, 0); width * height] }
    }

    /// Get a single pixel.
    pub fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }
        
    /// Blit the buffer to the framebuffer.
    pub fn blit(&self, gop: &mut GraphicsOutput) -> Result<()> {
        gop.blt(BltOp::BufferToVideob { buffer: &self.pixels,src: BltRegion::Full,dest: (0, 0), dims: (self.width, self.height)})
    }
}