use ncurses as nc;

use types::{BoxResult, Dimension, Menu, MenuItem};

pub use self::colors::ColorPair;

pub struct Window {
    pointer: nc::WINDOW
}

// impl Window {
//     pub fn get_max_dimension(&self) -> Dimension {
//         let width = nc::getmaxx(self.pointer);
//         let height = nc::getmaxy(self.pointer);

//         Dimension {
//             width: width,
//             height: height
//         }
//     }
// }

mod colors {
    use super::nc;

#[derive(Debug, Copy, Clone)]
pub enum ColorPair {
    WhiteBlack = 1,
    BlackWhite = 2,
    GreenBlack = 3,
    RedBlack = 4
}

pub fn init() {
    /**
     * terminal colors:
     * black
     * red
     * green
     * yellow
     * blue
     * magenta
     * cyan
     * white
     */

    nc::init_pair(ColorPair::WhiteBlack as i16, nc::COLOR_WHITE, nc::COLOR_BLACK);
    nc::init_pair(ColorPair::BlackWhite as i16, nc::COLOR_BLACK, nc::COLOR_WHITE);
    nc::init_pair(ColorPair::GreenBlack as i16, nc::COLOR_GREEN, nc::COLOR_BLACK);
    nc::init_pair(ColorPair::RedBlack as i16, nc::COLOR_RED, nc::COLOR_BLACK);

    // debug!("{}", nc::pair_content(nc::COLOR_PAIR(1)));
}

}

pub struct TerminalRenderer {
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&self) -> BoxResult<()> {
        // Start ncurses
        nc::initscr();

        if !nc::has_colors() {
            nc::endwin();
            error!("Terminal does not support color");
            // return Err(Box::new("Terminal does not support color".to_string()));
        }
    
        // Allow colors
        nc::start_color();

        colors::init();

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

        Ok(())
    }

    // pub fn erase(&self) {
    //     nc::erase();
    // }

    // pub fn clear(&self) {
    //     nc::clear();
    // }

    pub fn refresh(&self) {
        nc::refresh();
    }

    // Block until user input
    pub fn getch(&self) -> i32 {
        nc::getch()
    }

    // pub fn new_menu_item(name: &str, description: &str) -> MenuItem {
    //     nc::new_item(name, description)
    // }

    // pub fn new_menu(items: &mut Vec<MenuItem>) -> Menu {
    //     nc::new_menu(items)
    // }

    // pub fn refresh_window(window: &Window) {
    //     nc::wrefresh(*window);
    // }

    // Allocate memory for window structure
    // to manipulate and update
    // pub fn new_window(x: i32, y: i32, width: i32, height: i32) -> Window {
    //     // Create the window
    //     let window = nc::newwin(height, width, y, x);

    //     // Box
    //     nc::box_(window, 0, 0);

    //     // Rfresh
    //     nc::wrefresh(window);

    //     Window {
    //         pointer: window
    //     }
    // }

    pub fn mvprintw(&self, x: i32, y: i32, s: &str) {
        nc::mvprintw(y, x, s);
    }

    pub fn mvaddch(&self, x: i32, y: i32, c: char) {
        nc::mvaddch(y, x, c as u64);
    }

    pub fn mvaddch_color(&self, x: i32, y: i32, c: char, color: ColorPair) {
        let attr = nc::COLOR_PAIR(color as i16);
        nc::attron(attr);
        nc::mvaddch(y, x, c as u64);
        nc::attron(attr);
    }

    // pub fn get_std_window(&self) -> Window {
    //     Window {
    //         pointer: nc::stdscr()
    //     }
    // }

    pub fn attron(attr: u64) {
        nc::attron(attr);
    }

    pub fn attroff(attr: u64) {
        nc::attroff(attr);
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        nc::endwin();
    }
}
