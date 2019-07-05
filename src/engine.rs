use std::fs::File;
use std::cell::RefCell;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use command_manager::CommandManager;
use config_manager::ConfigManager;
use renderer::Renderer;
use character::Player;
use types::{Dimension, BoxResult};
use map::{Map, Cell as MapCell};
use entity::Entity;

// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);

pub trait GameObject {

}

#[derive(Debug)]
pub struct Prop {

}

impl GameObject for Prop {

}


#[derive(Debug)]
pub struct Item {

}

impl GameObject for Item {

}

pub enum Event {
    Move(i32, i32),
    Quit
}

/// Main engine
pub struct Engine {
    renderer: Renderer,
    config_manager: ConfigManager,
    command_manager: CommandManager,
    current_map: Option<RefCell<Map>>,
    event_sender: std::sync::mpsc::Sender<Event>,
    event_receiver: std::sync::mpsc::Receiver<Event>,
    entities: RefCell<HashMap<String, Entity>>
}

impl Engine {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let renderer = Renderer::new();
        let player = Player::new();

        Self {
            renderer: renderer,
            command_manager: CommandManager::new(),
            config_manager: ConfigManager::new(),
            current_map: None,
            event_sender: sender,
            event_receiver: receiver,
            entities: RefCell::new(HashMap::new())
        }
    }

    pub fn init(&self) {
        // Renderer::init();

        self.renderer.init();

        InputManager::init(self.event_sender.clone());

        // self.player.set_x(32);
        // self.player.set_y(22);

        // let main_window = self.renderer.get_std_window();
        // let dimension = main_window.get_max_dimension();

        // debug!("Main window size {} {}", dimension.width, dimension.height);
        // Spawn input handler thread
        
        // First state is main menu
        // self.state_manager.change_state(Box::new(MainMenu::new()))
    }

    pub fn register_entity(&self, id: &str, entity: Entity) {
        self.entities.borrow_mut().insert(id.to_string(), entity);
    }

    pub fn run(&mut self) {
        // TODO/DECISION
        // Should time be handled in floating point or int
        let mut previous = Instant::now();
        // let mut lag = 0.0f64;
        let mut game_time = 0u64;

        let mut lag = 0.0;

        'main: loop {
            let current = Instant::now();
            let elapsed = current.duration_since(previous);
            previous = current;
            lag += elapsed.as_secs_f64();

            // Events
            // handle_events();
            // Update
            // while game_time < current {
            //     lag -= MS_PER_UPDATE;
                
            //     self.update();
            // }

            self.update(elapsed);

            self.render();

            let mut iter = self.event_receiver.try_iter();

            // Poll for events
            while let Some(event) = iter.next() {
                match event {
                    Event::Move(x, y) => {
                        let mut entities = self.entities.borrow_mut();
                        let player = entities.get_mut("player");

                        if let Some(player) = player {
                            player._move(x, y);
                        }
                    },
                    Event::Quit => {
                        break 'main;
                    }
                }
            }

            previous = current;
        }
    }

    /**
     * Update UI state and game logic
     */
    fn update(&mut self, dt: Duration) -> Result<(), Box<std::error::Error>> {

        // self.state_manager.update();

        Ok(())
    }

    fn render(&self) {
        // self.renderer.erase();

        let viewport_x = 20;
        let viewport_y = 20;
        
        if let Some(ref map) = self.current_map {
            let map = map.borrow();
            // Render the map
            // For each cell in the map
            let map_dimensions = map.get_dimensions();

            // debug!("{:?}", map.get_cells().iter().map(|cell| cell.glyph).collect::<String>());

            for y in 0..map_dimensions.height {
                for x in 0..map_dimensions.width {
                    let cell = map.get_cell_ref(x, y);
                    self.renderer.mvaddch(viewport_x + x, viewport_y + y, cell.get_glyph())
                }
            }
        }

        // Render props

        // Render items

        // Render entitys
        for (id, entity) in self.entities.borrow().iter() {
            self.render_entity(entity)
        }

        // self.renderer.mvprintw(30, 20, "#################");

        // for y in 21..30 {
        //     self.renderer.mvprintw(30, y, "#               #");
        // }

        // self.renderer.mvprintw(30, 30, "#################");

        // self.player.render(&self.renderer);

        self.renderer.refresh();
    
        for (id, entity) in self.entities.borrow().iter() {
            self.clear_entity(entity);
        }
    }

    fn render_entity(&self, entity: &Entity) {
        self.renderer.mvaddch(entity.x, entity.y, entity.glyph);
    }

    fn clear_entity(&self, entity: &Entity) {
        self.renderer.mvaddch(entity.x, entity.y, ' ');
    }

    pub fn load_map(&mut self, filename: &str) -> BoxResult<()> {
        let map = Map::open(filename)?;

        self.current_map = Some(RefCell::new(map));

        Ok(())
    }

    fn spawn_entity(&self, x: i32, y: i32, entity: Entity) -> BoxResult<()> {
        match self.current_map {
            Some(ref map) => {
                map.borrow_mut().spawn_entity(x, y, entity);

                return Ok(())
            }
            None => {
                return Err(Box::new(EngineError::NoLoadedMap));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

mod InputManager {
    use ncurses as nc;
    use std::thread;
    use engine::Event;

    pub fn init(event_sender: std::sync::mpsc::Sender<Event>) {
        // Input thread
        let input_manager = thread::spawn(move || {
            info!("Input manager thread started");

            loop {
                let input = nc::getch();

                let event = handle_input(input);

                if let Some(event) = event {
                    event_sender.send(event)
                        .expect("Failed to send event");
                }
            }
        });
    }

    pub fn handle_input(input: i32) -> Option<Event> {
        match input {
            119 => {
                // 'w
                Some(Event::Move(0, -1))
            }
            115 => {
                // 'd'
                Some(Event::Move(0, 1))
            }
            100 => {
                // 'd'
                Some(Event::Move(1, 0))
            }
            97 => {
                // 'a'
                Some(Event::Move(-1, 0))
            }
            113 => {
                // 'q'
                Some(Event::Quit)
            }
            _ => None
        }
    }
}

#[derive(Debug)]
enum EngineError {
    NoLoadedMap,
    EntityNotFound
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EngineError::NoLoadedMap => write!(f, "No map loaded"),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl std::error::Error for EngineError { }