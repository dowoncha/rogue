use std::borrow::BorrowMut;

use std::time::{Duration, Instant};

use game_state::{GameStateManager, MainMenu};

use renderer::Renderer;
use character::Player;

use ncurses as nc;

// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);
struct Map {

}

impl Map {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, renderer: &Renderer) {
        renderer.mvprintw(30, 20, "#################");

        for y in 21..30 {
            renderer.mvprintw(30, y, "#               #");
        }

        renderer.mvprintw(30, 30, "#################");
    }
}

/// Main engine
pub struct Game {
    renderer: Renderer,
    player: Player,
    map: Map,
    state_manager: GameStateManager,
    event_sender: std::sync::mpsc::Sender<String>,
    event_receiver: std::sync::mpsc::Receiver<String>
}

impl Game {
    pub fn new() -> Game {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        Game {
            renderer: Renderer::new(),
            player: Player::new(),
            map: Map::new(),
            state_manager: GameStateManager::new(),
            event_sender: sender,
            event_receiver: receiver
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

        self.player.set_x(32);
        self.player.set_y(22);

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

            self.event_sender.send(format!("key {}", input)).expect("Failed to send event");

            self.update();

                        // Render
            self.render();

            // self.renderer.mvprintw(1, 1, &format!("{}", input));
        }
    }

    fn handle_input(&mut self, input: i32) {
        self.renderer.mvprintw(1, 2, "Handling input");
        match input {
            119 => {
                // 'w'
                let player_y = self.player.y;
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
    }

    fn update(&mut self) {
        let event = self.event_receiver.recv().unwrap();

        self.renderer.mvprintw(1, 1, &format!("Event: {}", event));

        let mut args = event.split_whitespace();

        let command = args.nth(1).unwrap();

        match command {
            "key" => {
                let input = args.nth(2).unwrap().parse::<i32>().unwrap();
                self.handle_input(input);
            },
            _ => {
                debug!("Unrecognized command {}", command);
            }
        }

        // self.state_manager.update();
    }

    fn render(&self) {
        // self.renderer.clear();

        self.map.render(&self.renderer);

        self.player.render(&self.renderer);

        self.renderer.refresh();

        // nc::printw(&format!("Framerate: {}", )

        // self.state_manager.render();
    }

    pub fn new_player(&mut self) {

    }
}


