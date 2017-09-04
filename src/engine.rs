use std::borrow::BorrowMut;

use std::time::{Duration, Instant};

use game_state::{GameStateManager, MainMenu};

use renderer::Renderer;
use character::Player;

use ncurses as nc;

/// Main engine
pub struct Game {
    renderer: Renderer,
    player: Player,
    state_manager: GameStateManager,
}

impl Game {
    pub fn new() -> Game {
        Game {
            renderer: Renderer::new(),
            player: Player::new(),
            state_manager: GameStateManager::new(),
        }
    }

    pub fn init(&mut self) {
        // Renderer::init();
        self.renderer.init();

        // First state is main menu
        self.state_manager.change_state(box MainMenu::new())
    }

    pub fn run(&mut self) {
        let mut last_time = Instant::now();

        loop {
            let current = Instant::now();
            let elapsed = current.duration_since(last_time);

            // Events
            // Update
            self.update();
            // Render
            self.render();
            //
            last_time = current;
        }
    }

    fn update(&mut self) {
        self.state_manager.update();
    }

    fn render(&self) {
        self.state_manager.render();
    }

    pub fn new_player(&mut self) {

    }
}


