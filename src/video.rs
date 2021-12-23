use std::result::Result;

use crate::ffmpeg::is_ffmpeg_available;
use crate::ffmpeg::start_ffmpeg;
use crate::ffmpeg::FfmpegError;

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
const WIDTH_DEFAULT: u32 = 1024;
const HEIGHT_DEFAULT: u32 = 720;

pub struct Video {
    pub fps: u32,
    pub width: u32,
    pub height: u32,
}

impl Video {
    pub fn new() -> Video {
        Video {
            fps: FPS_DEFAULT,
            width: WIDTH_DEFAULT,
            height: HEIGHT_DEFAULT,
        }
    }

    pub fn with_fps<'a>(&'a mut self, fps: u32) -> &'a mut Video {
        self.fps = fps;
        self
    }

    pub fn with_dimensions<'a>(&'a mut self, width: u32, height: u32) -> &'a mut Video {
        self.width = width;
        self.height = height;
        self
    }

    pub fn render(&self, filename: &str, ffmpeg_path: &str) -> Result<(), RenderError> {
        is_ffmpeg_available(ffmpeg_path)?;
        println!("Rendering to {}", filename);
        let ffmpeg_process = start_ffmpeg(self, ffmpeg_path, filename).unwrap();
        Ok(())
    }
}
