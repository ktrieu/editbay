use image::RgbaImage;

use transform::FrameNum;

pub mod transform;

pub mod image_clip;

pub trait Clip {
    // Render frame number `frame` to the framebuffer `image`.
    fn render_frame(&self, frame: FrameNum, framebuffer: &mut RgbaImage);
}
