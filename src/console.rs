#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8,pub  u8, pub u8);

impl Color {
    pub fn x(&self) -> u8 {
        self.0
    }

    pub fn y(&self) -> u8 {
        self.1
    }

    pub fn z(&self) -> u8 {
        self.2
    }
}

#[derive(Copy, Clone)]
struct Tile {
    ch: char,
    fg: Color,
    bg: Color
}

pub struct Console {
    width: usize,
    height: usize,
    buffer: Vec<Tile>
}

impl Console {
    pub fn new(width: usize, height: usize) -> Self {
        let default_fg = Color(0xff, 0xff, 0xff);
        let default_bg = Color(0x00, 0x00, 0x00);

        Self {
            width: width,
            height: height,
            buffer: vec![Tile { ch: ' ', fg: default_fg, bg: default_bg }; width * height]
        }
    }

    pub fn clear(&mut self, ch: char, fg: Color, bg: Color) {
        self.buffer = self.buffer.iter().map(|tile| Tile { ch: ch, fg: fg, bg: bg }).collect();
    }

    pub fn draw_rect(
        &mut self, 
        x: usize, 
        y: usize,
        height: usize,
        width: usize,
        ch: char,
        fg: Option<Color>,
        bg: Option<Color>
    ) {
        // Check x, y, width, and height
        for _y in y..(y + height) {
            for _x in x..(x + width) {
                let current_tile = &mut self.buffer[_y * self.width + _x];
                let new_ch = if ch == '\0' { current_tile.ch } else { ch };

                let new_tile = Tile {
                    ch: new_ch,
                    fg: fg.unwrap_or(current_tile.fg),
                    bg: bg.unwrap_or(current_tile.bg)
                };

                *current_tile = new_tile;
            }
        }
    }

    pub fn print(
        &mut self,
        x: usize,
        y: usize,
        line: &str,
        fg: Option<Color>,
        bg: Option<Color>
    ) {

    }

    pub fn put_char(
        &mut self,
        x: usize,
        y: usize,
        ch: char
    ) {
        self.buffer[y * self.width + x].ch = ch;
    }
}

pub use self::lib_console::init_root;

mod lib_console {
    use super::Console;

    pub fn init_root(
        width: usize,
        height: usize,
    ) -> Console {
        Console::new(width, height)
    }
}