use std::fmt::Display;
use std::io::ErrorKind;
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub enum FfmpegError {
    NotFound(String),
    Unknown(std::io::Error),
}

impl std::error::Error for FfmpegError {}

impl Display for FfmpegError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfmpegError::NotFound(path) => {
                write!(f, "Could not find FFMPEG at {}. Check your PATH.", path)
            }
            FfmpegError::Unknown(error) => {
                write!(f, "Unknown error starting FFMPEG: {}", error)
            }
        }
    }
}

impl From<std::io::Error> for FfmpegError {
    fn from(error: std::io::Error) -> Self {
        FfmpegError::Unknown(error)
    }
}

pub fn is_ffmpeg_available(ffmpeg_path: &str) -> Result<(), FfmpegError> {
    match Command::new(ffmpeg_path).output() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(FfmpegError::NotFound(ffmpeg_path.to_string()))
            } else {
                Err(FfmpegError::Unknown(e))
            }
        }
    }
}

pub fn start_ffmpeg(
    fps: i32,
    ffmpeg_path: &str,
    output_filename: &str,
) -> Result<Child, FfmpegError> {
    is_ffmpeg_available(ffmpeg_path)?;
    let subprocess = Command::new(ffmpeg_path)
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
        .spawn()?;
    Ok(subprocess)
}
