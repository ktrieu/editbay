use std::result::Result;

use crate::ffmpeg::start_ffmpeg;
use crate::ffmpeg::submit_frame;
use crate::ffmpeg::wait_ffmpeg;
use crate::ffmpeg::FfmpegError;
use crate::segment::Segment;
use crate::time::Time;

use indicatif::{ProgressBar, ProgressStyle};

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

    fn get_total_frames(&self) -> u32 {
        self.segments
            .iter()
            .map(|s| s.length.to_frame_num(self))
            .sum()
    }

    fn make_render_progress(&self) -> ProgressBar {
        let total_frames = self.get_total_frames();
        let progress_style = ProgressStyle::default_bar()
            .template("{pos:>7} frames / {len:7} {wide_bar} {per_sec:2} {elapsed} elapsed");
        ProgressBar::new(total_frames.into()).with_style(progress_style)
    }

    pub fn render(&self, filename: &str, ffmpeg_path: &str) -> Result<(), RenderError> {
        println!("Rendering to {}", filename);
        let mut ffmpeg_process = start_ffmpeg(self, ffmpeg_path, filename)?;
        let mut image = image::RgbaImage::new(self.width, self.height);
        let progress = self.make_render_progress();

        for s in &self.segments {
            let num_frames = s.length.to_frame_num(self);
            for frame in 0..num_frames {
                image.fill(0);

                for c in &s.clips {
                    c.render_frame(self, Time::Frame(frame), &mut image);
                }

                progress.inc(1);
                submit_frame(&mut ffmpeg_process, &image)?;
            }
        }

        wait_ffmpeg(&mut ffmpeg_process)?;
        progress.finish();
        Ok(())
    }
}
