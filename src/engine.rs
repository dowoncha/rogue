// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);


use std::cell::RefCell;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use rand::prelude::*;
use ncurses as nc;

use action::Action;
use config_manager::ConfigManager;
use renderer::{TerminalRenderer, ColorPair};
use character::Player;
use types::{Dimension, BoxResult, Rect};
use map::{Map, MapBuilder, Cell as MapCell};
use entity::Entity;

pub struct World {
    entities: Entities,
    current_map: Option<Map>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            current_map: None
        }
    }

    pub fn is_tile_occupied(&self, x: i32, y: i32) -> Option<&dyn Entity> {
        None
    }

    pub fn register_entity(&mut self, id: &str, mut entity: Box<dyn Entity>) {
        // entity.set_world(self);
        self.entities.insert(id.to_string(), entity);
    }

    fn is_cell_blocked(&self, x: i32, y: i32) -> bool {
        if let Some(map) = self.current_map.as_ref() {
            map.is_blocked(x, y) 
        } else {
            false
        }
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn get_entity(&self, entity_id: &str) -> Option<&Box<Entity>> {
        self.entities.get(entity_id)
    }

    pub fn get_current_map(&self) -> Option<&Map> {
        self.current_map.as_ref()
    }

    pub fn set_map(&mut self, map: Map) {
        self.current_map = Some(map);
    }
}

pub type Entities = HashMap<String, Box<dyn Entity>>;

/// Main engine
pub struct Engine {
    world: Box<World>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            // renderer: renderer,
            // config_manager: ConfigManager::new(),
            // current_map: None,
            // entities: HashMap::new(),
            world: Box::new(World::new())
        }
    }
    // POST /maps/generate

    pub fn make_map(
        &mut self, 
        max_rooms: i32, 
        room_min_size: usize, 
        room_max_size: usize, 
        map_width: usize, 
        map_height: usize
    ) -> (Vec<Rect>, Map) {
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
                // connect it to the previous room with a tunnel
                if !rooms.is_empty() {
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

        (rooms, map)
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_mut_world(&mut self) -> &mut World {
        &mut self.world
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