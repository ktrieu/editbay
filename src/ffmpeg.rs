use std::fmt::Display;
use std::io::{ErrorKind, Write};
use std::process::{Child, Command, Stdio};

use image::{EncodableLayout, RgbaImage};

use crate::video::Video;

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

fn get_format_args() -> [&'static str; 4] {
    return ["-f", "rawvideo", "-pix_fmt", "rgba"];
}

fn get_dimension_args(video: &Video) -> [String; 2] {
    return [
        "-s".to_string(),
        format!("{}x{}", video.width, video.height),
    ];
}

fn get_fps_args(video: &Video) -> [String; 2] {
    ["-r".to_string(), video.fps.to_string()]
}

fn get_input_args() -> [&'static str; 2] {
    ["-i", "-"]
}

fn get_output_args(output_filename: &str) -> [&str; 1] {
    [output_filename]
}

pub fn start_ffmpeg(
    video: &Video,
    ffmpeg_path: &str,
    output_filename: &str,
) -> Result<Child, FfmpegError> {
    is_ffmpeg_available(ffmpeg_path)?;
    let subprocess = Command::new(ffmpeg_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-y") // overwrite the output file without asking
        .args(get_format_args())
        .args(get_dimension_args(video))
        .args(get_fps_args(video))
        .args(get_input_args())
        .args(get_fps_args(video))
        .args(get_output_args(output_filename))
        .spawn()?;
    Ok(subprocess)
}

pub fn submit_frame(subprocess: &mut Child, frame: &RgbaImage) -> Result<(), FfmpegError> {
    // we always capture stdin so it'll always be there
    let stdin = subprocess.stdin.as_mut().unwrap();
    stdin.write_all(frame.as_bytes())?;
    Ok(())
}

pub fn wait_ffmpeg(subprocess: &mut Child) -> Result<(), FfmpegError> {
    subprocess.wait()?;
    Ok(())
}
