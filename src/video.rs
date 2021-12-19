use std::result::Result;

use crate::ffmpeg::is_ffmpeg_available;
use crate::ffmpeg::FfmpegSearchError;

pub struct Video {
    fps: i32,
}

#[derive(Debug)]
pub enum RenderError {
    FfmpegError(FfmpegSearchError),
}

impl From<FfmpegSearchError> for RenderError {
    fn from(err: FfmpegSearchError) -> Self {
        RenderError::FfmpegError(err)
    }
}

const FPS_DEFAULT: i32 = 24;

impl Video {
    pub fn new() -> Video {
        Video { fps: FPS_DEFAULT }
    }

    pub fn with_fps<'a>(&'a mut self, fps: i32) -> &'a mut Video {
        self.fps = fps;
        self
    }

    pub fn render(&self, filename: &str, ffmpeg_path: &str) -> Result<(), RenderError> {
        is_ffmpeg_available(ffmpeg_path)?;
        println!("Rendering to {}", filename);
        Ok(())
    }
}
