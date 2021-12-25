use crate::{clip::Clip, time::Time};

pub struct Segment {
    pub length: Time,
    pub clips: Vec<Box<dyn Clip>>,
}

impl Segment {
    pub fn new() -> Self {
        Segment {
            length: Time::Seconds(0.),
            clips: Vec::new(),
        }
    }
}
