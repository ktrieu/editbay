use std::result::Result;

use crate::clip::transform::ClipTransform;
use crate::clip::Clip;
use crate::ffmpeg::start_ffmpeg;
use crate::ffmpeg::submit_frame;
use crate::ffmpeg::wait_ffmpeg;
use crate::ffmpeg::FfmpegError;

use crate::clip::image_clip::ImageClip;
use crate::clip::transform::Rect;
use crate::time::Duration;
use crate::time::Time;

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

        let transform = ClipTransform {
            duration: Duration {
                start: Time::Seconds(0.),
                end: Time::Seconds(10.),
            },
            bounds: Rect::from_dimensions(0, 0, 64, 64),
        };

        let clip = ImageClip::from_file(transform, "taco.jpg").unwrap();

        for frame in 0..Time::Seconds(20.).to_frame_num(self) {
            // clear the frame
            image.fill(0);
            clip.render_frame(self, Time::Frame(frame), &mut image);
            submit_frame(&mut ffmpeg_process, &image)?;
        }

        wait_ffmpeg(&mut ffmpeg_process)?;
        Ok(())
    }
}
