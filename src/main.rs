use editbay::video::RenderError;

pub fn main() -> Result<(), RenderError> {
    editbay::video::Video::new()
        .with_fps(32)
        .render("video.mp4", "ffmpeg")
}
