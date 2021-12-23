use std::result::Result;

use crate::clip::transform::ClipTransform;
use crate::clip::transform::FrameNum;
use crate::clip::Clip;
use crate::ffmpeg::start_ffmpeg;
use crate::ffmpeg::submit_frame;
use crate::ffmpeg::wait_ffmpeg;
use crate::ffmpeg::FfmpegError;

use crate::clip::image_clip::ImageClip;
use crate::clip::transform::Rect;

#[derive(Debug)]
pub enum RenderError {
    FfmpegError(FfmpegError),
}

impl std::error::Error for RenderError {}

impl From<FfmpegError> for RenderError {
    fn from(err: FfmpegError) -> Self {
        RenderError::FfmpegError(err)
    }
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::FfmpegError(e) => {
                write!(fmt, "FFMPEG Error: {}", e)
            }
        }
    }
}

const FPS_DEFAULT: u32 = 24;

pub struct Video {
    pub fps: u32,
    pub width: u32,
    pub height: u32,
}

impl Video {
    pub fn new(width: u32, height: u32) -> Video {
        Video {
            fps: FPS_DEFAULT,
            width: width,
            height: height,
        }
    }

    pub fn with_fps<'a>(&'a mut self, fps: u32) -> &'a mut Video {
        self.fps = fps;
        self
    }

    pub fn render(&self, filename: &str, ffmpeg_path: &str) -> Result<(), RenderError> {
        println!("Rendering to {}", filename);
        let mut ffmpeg_process = start_ffmpeg(self, ffmpeg_path, filename)?;
        let mut image = image::RgbaImage::new(self.width, self.height);

        let num_frames = FrameNum(self.fps * 4);
        let transform = ClipTransform {
            in_frame: FrameNum(0),
            // shitty hacks to get this to compile so i can test it
            out_frame: FrameNum(num_frames.0 / 2),
            bounds: Rect::from_dimensions(0, 0, 64, 64),
        };

        let clip = ImageClip::from_file(transform, "taco.jpg").unwrap();

        for frame in 0..num_frames.0 {
            // clear the frame
            image.fill(0);

            clip.render_frame(FrameNum(frame), &mut image);
            submit_frame(&mut ffmpeg_process, &image)?;
        }

        wait_ffmpeg(&mut ffmpeg_process)?;
        Ok(())
    }
}
