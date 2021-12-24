use image::imageops;
use image::io::Reader as ImageReader;
use image::RgbaImage;

use crate::clip::transform::ClipTransform;
use crate::clip::Clip;
use crate::time::Time;
use crate::video::Video;

#[derive(Debug)]
pub enum ImageClipError {
    FileLoadError {
        path: String,
        error: std::io::Error,
    },
    DecodeError {
        path: String,
        error: image::ImageError,
    },
}

impl std::error::Error for ImageClipError {}

impl std::fmt::Display for ImageClipError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageClipError::FileLoadError { path, error } => {
                write!(
                    fmt,
                    "Could not load image file {}. IO error: {}",
                    path, error
                )
            }
            ImageClipError::DecodeError { path, error } => {
                write!(
                    fmt,
                    "Could not decode image file {}. Image error: {}",
                    path, error
                )
            }
        }
    }
}

pub struct ImageClip {
    transform: ClipTransform,
    image: RgbaImage,
}

impl ImageClip {
    pub fn from_file(transform: ClipTransform, path: &str) -> Result<Self, ImageClipError> {
        let reader = ImageReader::open(path).map_err(|e| ImageClipError::FileLoadError {
            path: path.to_string(),
            error: e,
        })?;
        let mut loaded = reader
            .decode()
            .map_err(|e| ImageClipError::DecodeError {
                path: path.to_string(),
                error: e,
            })?
            .to_rgba8();

        // resize the image to the scale we know we'll be rendering it at
        // so we can do a fast copy per frame
        let width = transform.width();
        let height = transform.height();

        loaded = imageops::resize(&loaded, width, height, imageops::FilterType::Triangle);

        Ok(ImageClip {
            transform: transform,
            image: loaded,
        })
    }
}

impl Clip for ImageClip {
    fn render_frame(&self, video: &Video, time: Time, framebuffer: &mut RgbaImage) {
        if !self.transform.is_active(video, time) {
            return;
        }
        // the image is already the right size so just copy it at the given location
        imageops::overlay(
            framebuffer,
            &self.image,
            self.transform.x(),
            self.transform.y(),
        );
    }
}
