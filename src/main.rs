use editbay::video::RenderError;

pub fn main() -> Result<(), RenderError> {
    editbay::video::Video::new(1080, 720)
        .with_fps(32)
        .render("video.mp4", "ffmpeg")
}
