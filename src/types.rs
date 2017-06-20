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