use std::fs::File;
use std::cell::RefCell;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use input_manager;
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

    pub fn init(
        &mut self, 
        screen_w: usize, 
        screen_h: usize
    ) {
        self.renderer.init();

        input_manager::init(self.event_sender.clone());

        // Initialize the map
        let map = Map::new(screen_w, screen_h);

        self.current_map = Some(RefCell::new(map));

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

    pub fn run(&mut self) -> BoxResult<()> {
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
                    Event::Move(dx, dy) => {
                        let mut entities = self.entities.borrow_mut();
                        let player = entities.get_mut("player");

                        if let Some(player) = player {
                            let map = self.current_map.as_ref()
                                .expect("No current map in engine").borrow();
                            let blocked = map.is_blocked(player.x + dx, player.y + dy);
                            if !blocked {
                                player._move(dx, dy);
                            }
                        }
                    },
                    Event::Quit => {
                        break 'main;
                    }
                }
            }

            previous = current;
        }

        Ok(())
    }

    /**
     * Update UI state and game logic
     */
    fn update(&mut self, dt: Duration) -> Result<(), Box<std::error::Error>> {

        // self.state_manager.update();

        Ok(())
    }

    fn render(&self) {
        let viewport_x = 0;
        let viewport_y = 0;
        
        if let Some(ref map) = self.current_map {
            let map = map.borrow();
            // Render the map
            // For each cell in the map
            let map_dimensions = map.get_dimensions();

            // debug!("{:?}", map.get_cells().iter().map(|cell| cell.glyph).collect::<String>());

            for y in 0..map_dimensions.height {
                for x in 0..map_dimensions.width {
                    let cell = map.get_cell_ref(x, y);
                    self.renderer.mvaddch(viewport_x + x, viewport_y + y, cell.glyph)
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
}

#[cfg(test)]
mod tests {
    use super::*;
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