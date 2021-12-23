#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FrameNum(pub u32);

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn from_dimensions(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {
            min: Point { x: x, y: y },
            max: Point {
                x: x + width,
                y: y + height,
            },
        }
    }

    pub fn x(&self) -> u32 {
        self.min.x
    }

    pub fn y(&self) -> u32 {
        self.min.y
    }

    pub fn width(&self) -> u32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> u32 {
        self.max.y - self.min.y
    }
}

pub struct ClipTransform {
    pub in_frame: FrameNum,
    pub out_frame: FrameNum,
    pub bounds: Rect,
}

impl ClipTransform {
    pub fn is_active(&self, frame: FrameNum) -> bool {
        self.in_frame <= frame && frame <= self.out_frame
    }

    pub fn x(&self) -> u32 {
        self.bounds.x()
    }

    pub fn y(&self) -> u32 {
        self.bounds.y()
    }

    pub fn width(&self) -> u32 {
        self.bounds.width()
    }

    pub fn height(&self) -> u32 {
        self.bounds.height()
    }
}
