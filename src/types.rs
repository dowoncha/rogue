use ncurses as nc;

pub struct Dimension {
    pub width: i32,
    pub height: i32
}

pub type MenuItem = nc::ITEM;
pub type Menu = nc::MENU;
#[derive(Debug)]
pub struct Color {
    r: f32, 
    g: f32, 
    b: f32
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 0f32,
            g: 0f32,
            b: 0f32
        }
    }
}

pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height
        }
    }
}

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;