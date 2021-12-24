use crate::video::Video;

#[derive(Debug)]
pub enum Time {
    Frame(u32),
    Seconds(f32),
}

impl Time {
    pub fn to_frame_num(&self, video: &Video) -> u32 {
        match self {
            Time::Frame(frame) => *frame,
            Time::Seconds(secs) => (secs * video.fps as f32).trunc() as u32,
        }
    }
}

#[derive(Debug)]
pub struct Duration {
    pub start: Time,
    pub end: Time,
}

impl Duration {
    pub fn contains(&self, time: Time, video: &Video) -> bool {
        let start_frame = self.start.to_frame_num(video);
        let end_frame = self.end.to_frame_num(video);
        let frame = time.to_frame_num(video);
        start_frame <= frame && frame <= end_frame
    }
}
