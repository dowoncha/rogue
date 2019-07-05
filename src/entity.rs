pub struct EntityBuilder {
    name: Option<String>
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            name: None
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());

        self
    }

    pub fn build(self) -> Entity {
        let name = self.name.unwrap_or("entity1".to_string());

        Entity {
            x: 0,
            y: 0,
            glyph: ' ',
            color: 0
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: char,
    color: u8
}

impl Entity {
    pub fn new(x: i32, y: i32, glyph: char, color: u8) -> Self {
        Self {
            x: x,
            y: y,
            glyph: glyph,
            color: color
        }
    }

    pub fn _move(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

// impl GameObject for Entity {

// }
