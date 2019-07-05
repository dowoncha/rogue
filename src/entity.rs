use renderer::ColorPair;

pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: char,
    pub color: ColorPair,
    components: Vec<Box<Component>>
}

trait Component {

}

#[derive(Debug)]
struct PositionComponent {
    x: i32,
    y: i32
}

impl Component for PositionComponent {

}

#[derive(Debug)]
struct RenderComponent {
    glyph: char,
    color: ColorPair
}

impl Component for RenderComponent {

}

struct CollideComponent {
    blocks: bool
}

impl Component for CollideComponent {

}

impl Entity {
    pub fn new(x: i32, y: i32, glyph: char, color: Option<ColorPair>) -> Self {
        let position = Box::new(PositionComponent {
            x: x,
            y: y
        });

        let render = Box::new(RenderComponent {
            glyph: glyph,
            color: color.unwrap_or(ColorPair::WhiteBlack)
        });

        Self {
            x: x,
            y: y,
            glyph: glyph,
            color: color.unwrap_or(ColorPair::WhiteBlack),
            components: vec![position, render]
        }
    }

    pub fn add_component(&mut self, component: Box<Component>) {
        self.components.push(component);
    }

    pub fn _move(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

// impl GameObject for Entity {

// }
