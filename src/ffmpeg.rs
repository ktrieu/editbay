use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::ErrorKind;
use std::process::{Child, Command, Stdio};

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

pub fn is_ffmpeg_available(ffmpeg_path: &str) -> Result<(), FfmpegSearchError> {
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

pub fn start_ffmpeg(fps: i32, ffmpeg_path: &str, output_filename: &str) -> io::Result<Child> {
    Command::new(ffmpeg_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([
            "-f",
            "image2pipe",
            "-c:v",
            "ppm",
            "-i",
            "-",
            "-r",
            &fps.to_string(),
            output_filename,
        ])
        .spawn()
}
