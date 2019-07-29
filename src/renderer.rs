use ncurses as nc;

use types::Rect;

pub trait Widget {
    fn render(&self);
}

pub struct Element {
    children: Vec<Box<dyn Widget>>
}

pub struct Button {

}

pub trait Renderer {
    fn new() -> Self;

    fn render_str(&self, x: i32, y: i32, text: &str);

    fn render_rect(&self, rect: &Rect);

    fn render_widget(&self, widget: impl Widget);
}

pub struct NcursesRenderer {

}


impl Renderer for NcursesRenderer {
    fn new() -> Self {
        Self {

        }
    }

    fn render_str(&self, x: i32, y: i32, text: &str) {
        nc::mvwaddstr(nc::stdscr(), y, x, text);
    }

    fn render_rect(&self, rect: &Rect) {
    }

    fn render_widget(&self, widget: impl Widget) {

    }
}

pub struct TestRenderer;

impl Renderer for TestRenderer {
    fn new() -> Self {
        Self
    }

    fn render_str(&self, x: i32, y: i32, text: &str) {

    }

    fn render_rect(&self, rect: &Rect) {

    }

    fn render_widget(&self, widget: impl Widget) {

    }
}