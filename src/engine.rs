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
        // self.state_manager.change_state(Box::new(MainMenu::new()))
    }

    pub fn run(&mut self) {
        let mut previous = Instant::now();
        // let mut lag = 0.0f64;
        let mut game_time = 0u64;

        self.player.set_x(20);
        self.player.set_y(40);

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
            let input = self.renderer.getch();


            match input {
                119 => {
                    let player_y = self.player.y;
                    // 'w'
                    self.player.set_y(player_y - 1);
                },
                115 => {
                    // 'd'
                    let player_y = self.player.y;

                    self.player.set_y(player_y + 1);
                },
                100 => {
                    // 'd'
                    let player_x = self.player.x;
                    self.player.set_x(player_x + 1);
                },
                97 => {
                    // 'a'
                    let player_x = self.player.x;

                    self.player.set_x(player_x - 1);
                },
                113 => {
                    // 'q'
                    return;
                },
                _ => {}
            }

            // match input {
            //     Some(nc::WchResult::Char(ch)) => {
            //         let ascii = std::char::from_u32(ch);

            //         if let Some(ascii) = ascii {
            //             match ascii {
            //                 'w' => {
            //                     player_y -= 1;
            //                 },
            //                 _ => {}
            //             }
            //         }
            //     },
            //     _ => {}
            // }
            
            // Render
            self.render();

            self.renderer.mvprintw(1, 1, &format!("{}", input));
        }
    }

    fn update(&mut self) {
        self.state_manager.update();
    }

    fn render(&self) {
        self.renderer.clear();

        nc::mvaddch(self.player.y, self.player.x, '@' as u64);

        self.renderer.refresh();

        // nc::printw(&format!("Framerate: {}", )

        // self.state_manager.render();
    }

    pub fn new_player(&mut self) {

    }
}


