use std::result::Result;

use crate::ffmpeg::start_ffmpeg;
use crate::ffmpeg::submit_frame;
use crate::ffmpeg::wait_ffmpeg;
use crate::ffmpeg::FfmpegError;
use crate::segment::Segment;
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
    pub segments: Vec<Segment>,
}

impl Video {
    pub fn new(width: u32, height: u32) -> Video {
        Video {
            fps: FPS_DEFAULT,
            width: width,
            height: height,
            segments: Vec::new(),
        }
    }

    pub fn with_fps<'a>(mut self, fps: u32) -> Video {
        self.fps = fps;
        self
    }

    pub fn get_total_frames(&self) -> u32 {
        self.segments
            .iter()
            .map(|s| s.length.to_frame_num(self))
            .sum()
    }

    pub fn render<F: Fn(u32)>(
        &self,
        filename: &str,
        ffmpeg_path: &str,
        progress_callback: F,
    ) -> Result<(), RenderError> {
        println!("Rendering to {}", filename);
        let mut ffmpeg_process = start_ffmpeg(self, ffmpeg_path, filename)?;
        let mut image = image::RgbaImage::new(self.width, self.height);

        let mut frames_processed = 0;

        for s in &self.segments {
            let num_frames = s.length.to_frame_num(self);
            for frame in 0..num_frames {
                image.fill(0);

                for c in &s.clips {
                    c.render_frame(self, Time::Frame(frame), &mut image);
                }
                frames_processed += 1;
                progress_callback(frames_processed);
                submit_frame(&mut ffmpeg_process, &image)?;
            }
        }

        wait_ffmpeg(&mut ffmpeg_process)?;
        Ok(())
    }
}
