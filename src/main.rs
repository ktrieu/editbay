use editbay::clip::image_clip::ImageClip;
use editbay::clip::transform::{ClipTransform, Rect};
use editbay::segment::Segment;
use editbay::time::{Duration, Time};
use editbay::video::RenderError;

pub fn main() -> Result<(), RenderError> {
    let mut video = editbay::video::Video::new(1080, 720).with_fps(32);

    let mut segment = Segment::new();
    segment.length = Time::Seconds(20.);

    let transform = ClipTransform {
        bounds: Rect::from_dimensions(0, 0, 500, 500),
        duration: Duration {
            start: Time::Seconds(0.),
            end: Time::Seconds(10.),
        },
    };
    let transform2 = ClipTransform {
        bounds: Rect::from_dimensions(250, 250, 100, 100),
        duration: Duration {
            start: Time::Seconds(5.),
            end: Time::Seconds(15.),
        },
    };

    let image_clip1 = ImageClip::from_file(transform, "taco.jpg").unwrap();
    let image_clip2 = ImageClip::from_file(transform2, "taco.jpg").unwrap();

    segment.clips.push(Box::new(image_clip1));
    segment.clips.push(Box::new(image_clip2));

    video.segments.push(segment);
    video.render("video.mp4", "ffmpeg")?;
    Ok(())
}
