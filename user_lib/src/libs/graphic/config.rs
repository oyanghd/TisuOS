use core::ops::Sub;

pub const WIDTH     : usize = 640;
pub const HEIGHT    : usize = 480;
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x1 : u32,
    pub y1 : u32,
    pub x2 : u32,
    pub y2 : u32,
}

pub struct Area {
    pub x1 : usize,
    pub y1 : usize,
    pub x2 : usize,
    pub y2 : usize,
}

impl Area {
    pub fn new(x1:usize, y1:usize, x2:usize, y2:usize)->Self {
        Self {
            x1,
            y1,
            x2,
            y2,
        }
    }

    pub fn inside(&self, point : Position)->bool {
        self.x1 <= point.x && self.y1 <= point.y && self.x2 >= point.x && self.y2 >= point.y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x : usize,
    pub y : usize,
}

impl Position {
    pub fn new(x : usize, y : usize)->Self {
        Self {
            x,
            y,
        }
    }
    pub fn new_default()->Self {
        Self {
            x : 0,
            y : 0,
        }
    }
    pub fn from_scale_point(p : ScalePoint)->Self {
        Self {
            x : (p.x * WIDTH as f32) as usize,
            y : (p.y * HEIGHT as f32) as usize,
        }
    }
}

/// ## 比例坐标点
/// 0 ~ 1，表示在屏幕中的位置比例
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalePoint {
    pub x : f32,
    pub y : f32,
}

impl ScalePoint {
    pub const fn default()->Self{
        Self{
            x : 0.0,
            y : 0.0,
        }
    }

    pub fn new(x:usize, y:usize)->Self {
        Self {
            x:x as f32 / 65535.0,
            y:y as f32 / 65535.0,
        }
    }
}

impl Sub for ScalePoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x : isize,
    pub y : isize,
}

impl Vector {
    pub fn new(x : isize, y : isize)->Self {
        Self {
            x : x,
            y : y,
        }
    }
    pub fn from(point : ScalePoint)->Self {
        Self {
            x : (point.x * WIDTH as f32) as isize,
            y : (point.y * HEIGHT as f32) as isize,
        }
    }
}
