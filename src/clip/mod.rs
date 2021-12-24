use crate::{time::Time, video::Video};
use image::RgbaImage;

pub mod transform;

pub mod image_clip;

pub trait Clip {
    // Render frame number `frame` to the framebuffer `image`.
    fn render_frame(&self, video: &Video, time: Time, framebuffer: &mut RgbaImage);
}
