use super::{Cb64, Record};

#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
pub struct Framebuffer {
    pub record: Record,
    pub physical_address: Cb64,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub bytes_per_line: u32,
    pub bits_per_pixel: u8,
    pub red_mask_pos: u8,
    pub red_mask_size: u8,
    pub green_mask_pos: u8,
    pub green_mask_size: u8,
    pub blue_mask_pos: u8,
    pub blue_mask_size: u8,
    pub reserved_mask_pos: u8,
    pub reserved_mask_size: u8,
}
