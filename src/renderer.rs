use ncurses as nc;

use types::{Menu, Window, MenuItem};

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {

        }
    }

    pub fn init(&self) {
        // Start ncurses
        nc::initscr();
    
        // Allow colors
        nc::start_color();

        // Line buffering disabled
        // Signals are not interpreted and are instead passed directly to program
        // TODO: change to raw after implementing signals
        // nc::raw();
        nc::cbreak();

        // Disable echoing of chracaters
        nc::noecho();

        // Enableds reading of function keys
        nc::keypad(nc::stdscr(), true);

        nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    pub fn clear(&self) {
        nc::clear();
    }

    pub fn refresh(&self) {
        nc::refresh();
    }

    // Block until user input
    pub fn getch(&self) -> i32 {
        nc::getch()
    }

    pub fn new_menu_item(name: &str, description: &str) -> MenuItem {
        nc::new_item(name, description)
    }

    pub fn new_menu(items: &mut Vec<MenuItem>) -> Menu {
        nc::new_menu(items)
    }

    pub fn refresh_window(window: &Window) {
        nc::wrefresh(*window);
    }

    // Allocate memory for window structure
    // to manipulate and update
    pub fn new_window(x: i32, y: i32, width: i32, height: i32) -> Window {
        // Create the window
        let window = nc::newwin(height, width, y, x);

        // Box
        nc::box_(window, 0, 0);

        // Rfresh
        nc::wrefresh(window);

        window
    }

    pub fn mvprintw(&self, x: i32, y: i32, c: &str) {
        nc::mvprintw(x, y, c);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        nc::endwin();
    }
}
