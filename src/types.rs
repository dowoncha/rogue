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

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;