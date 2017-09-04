use std::borrow::BorrowMut;

use std::time::{Duration, Instant};

use game_state::{GameStateManager, MainMenu};

use renderer::Renderer;
use character::Player;

use ncurses as nc;

// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);

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
        let mut previous = Instant::now();
        // let mut lag = 0.0f64;
        let mut game_time = 0u64;

        loop {
            //let current = Instant::now();
            // let elapsed = current.duration_since(current);
            // previous = current;
            // lag += elapsed;

            // Events
            // handle_events();
            // Update
            /*
            while game_time < current {
                lag -= MS_PER_UPDATE;
                
                self.update();
            }
            */
            
            // Render
            self.render();
        }
    }

    fn update(&mut self) {
        self.state_manager.update();
    }

    fn render(&self) {
        // nc::printw(&format!("Framerate: {}", )

        self.state_manager.render();
    }

    pub fn new_player(&mut self) {

    }
}


