use std::fmt;
use std::fmt::Display;
use std::io::ErrorKind;
use std::process::Command;
use std::result::Result;

#[derive(Debug)]
pub enum FfmpegSearchError {
    NotFound(String),
    Unknown(String),
}

impl Display for FfmpegSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfmpegSearchError::NotFound(path) => {
                write!(f, "Could not find FFMPEG at {}. Check your PATH.", path)
            }
            FfmpegSearchError::Unknown(error) => {
                write!(f, "Unknown error locating FFMPEG: {}", error)
            }
        }
    }
}

fn is_ffmpeg_available(ffmpeg_path: &str) -> Result<(), FfmpegSearchError> {
    match Command::new(ffmpeg_path).output() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(FfmpegSearchError::NotFound(ffmpeg_path.to_string()))
            } else {
                Err(FfmpegSearchError::Unknown(e.to_string()))
            }
        }
    }
}

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
