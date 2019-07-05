use std::fs::File;
use std::cell::RefCell;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use rand::prelude::*;
use ncurses as nc;

use input_manager;
use command_manager::CommandManager;
use config_manager::ConfigManager;
use renderer::{TerminalRenderer, ColorPair};
use character::Player;
use types::{Dimension, BoxResult, Rect};
use map::{Map, MapBuilder, Cell as MapCell};
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
    renderer: TerminalRenderer,
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

        let renderer = TerminalRenderer::new();
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

        let fov_radius = 10;

        // let main_window = self.renderer.get_std_window();
        // let dimension = main_window.get_max_dimension();

        // debug!("Main window size {} {}", dimension.width, dimension.height);
        // Spawn input handler thread
        
        // First state is main menu
        // self.state_manager.change_state(Box::new(MainMenu::new()))
    }

    pub fn make_map(
        &mut self, 
        max_rooms: i32, 
        room_min_size: usize, 
        room_max_size: usize, 
        map_width: usize, 
        map_height: usize
    ) -> Vec<Rect> {
        let mut rng = rand::thread_rng();

        let mut map_builder = MapBuilder::new(map_width, map_height);

        let mut rooms = Vec::new();

        for _ in 0..max_rooms {
            let w = rng.gen_range(room_min_size, room_max_size) as i32;
            let h = rng.gen_range(room_min_size, room_max_size) as i32;

            let x = rng.gen_range(0, map_width as i32 - w - 1);
            let y = rng.gen_range(0, map_height as i32 - h - 1 );

            let new_room = Rect::new(x, y, w, h);

            let mut intersected = false;

            for other_room in &rooms {
                if new_room.intersect(other_room) {
                    intersected = true;
                    break;
                }
            }

            if !intersected {
                map_builder = map_builder.create_room(&new_room);

                let (new_room_center_x, new_room_center_y ) = new_room.center();


                // TODO:
                // this is a side effect and should be moved elsewhere
                // If this the first room, put the player in it
                if rooms.is_empty() {
                    let mut entities = self.entities.borrow_mut();
                    let player = entities.get_mut("player");
                    if let Some(p) = player {
                        p.x = new_room_center_x;
                        p.y = new_room_center_y;
                    }
                } else {
                    // All rooms after the first
                    // connect it to the previous room with a tunnel
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    // flip a coin
                    if rng.gen::<f32>() > 0.5 {
                        map_builder = map_builder
                            .create_h_tunnel(prev_x, new_room_center_x, prev_y)
                            .create_v_tunnel(prev_y, new_room_center_y, new_room_center_x);
                    } else {
                        map_builder = map_builder
                            .create_v_tunnel(prev_y, new_room_center_y, prev_x)
                            .create_h_tunnel(prev_x, new_room_center_x, new_room_center_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        let map = map_builder.build();

        self.current_map = Some(RefCell::new(map));

        rooms
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
                        self.move_entity("player", dx, dy);
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

    fn move_entity(&self, entity_id: &str, dx: i32, dy: i32) {
        let mut entities = self.entities.borrow_mut();
        let entity = entities.get_mut(entity_id);

        if let Some(entity) = entity {
            if !self.is_entity_blocked(entity, dx, dy) {
                entity._move(dx, dy);
            }
        }
    }

    fn is_entity_blocked(&self, entity: &Entity, dx: i32, dy: i32) -> bool {
        let map = self.current_map.as_ref()
                .expect("No current map in engine")
                .borrow();

        map.is_blocked(entity.x + dx, entity.y + dy)
    }

    /**
     * Update UI state and game logic
     */
    fn update(&mut self, dt: Duration) -> Result<(), Box<std::error::Error>> {

        // self.state_manager.update();

        Ok(())
    }

    fn render(&self) {
        self.render_map();

        // Render entitys
        self.render_entities();

        self.renderer.refresh();

        self.clear_entities();
    }

    fn render_map(&self) {
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
                    if (cell.glyph == '#') {
                        // let attr = nc::COLOR_PAIR(colors::ColorPair::WhiteBlack as i16);
                        // let attr = nc::COLOR_PAIR(1);
                        // nc::attron(attr);
                        self.renderer.mvaddch_color(viewport_x + x, viewport_y + y, cell.glyph, ColorPair::WhiteBlack);
                        // nc::attroff(attr);
                    } else {
                        self.renderer.mvaddch(viewport_x + x, viewport_y + y, cell.glyph)
                    }
                }
            }
        }
    }

    fn render_entities(&self) {
        for (id, entity) in self.entities.borrow().iter() {
            self.render_entity(entity)
        }
    }

    fn render_entity(&self, entity: &Entity) {
        self.renderer.mvaddch_color(entity.x, entity.y, entity.glyph, entity.color);
    }

    fn clear_entities(&self) {
        for (id, entity) in self.entities.borrow().iter() {
            self.clear_entity(entity);
        }
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