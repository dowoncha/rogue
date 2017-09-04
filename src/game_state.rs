use ncurses as nc;

use std::borrow::Borrow;

use types::{Menu, Window, MenuItem};
use renderer::Renderer;

pub struct GameStateManager {
    states: Vec<Box<GameState>>
}

impl GameStateManager {
    pub fn new() -> GameStateManager {
        GameStateManager {
            states: Vec::new()
        }
    }

    pub fn update(&mut self) {
        if let Some(state) = self.states.last_mut() {
            state.update();
        }    
    }

    pub fn render(&self) {
        self.get_current_state().render();
    }

    fn get_current_state(&self) -> &GameState {
        self.states.last().unwrap().borrow()
    }

    pub fn handle_events(&self) {
        self.get_current_state().handle_events();
    }

    pub fn change_state(&mut self, state: Box<GameState>) {
        self.states.push(state);
    }
}

pub trait GameState {
    fn init(&mut self) { }

    fn pause(&self) { }

    fn resume(&self) { }

    fn update(&mut self) { }

    fn render(&self) { }

    fn handle_events(&self) { }
}

pub struct MainMenu {
    menu: Menu,
    window: Window,
    items: Vec<MenuItem>
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut items = vec!(
            Renderer::new_menu_item("Choice 1", "Choice 1 description")
        );

        let menu = Renderer::new_menu(&mut items);

        let window = Renderer::new_window(9, 18, 4, 4);

        MainMenu {
            items,
            menu,
            window
        }
    }
}

impl GameState for MainMenu {
    fn render(&self) {
        nc::printw("Welcome to Rogue");
    }

    fn handle_events(&self) {
    }
}
