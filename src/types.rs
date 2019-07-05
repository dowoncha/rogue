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

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1
            && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;