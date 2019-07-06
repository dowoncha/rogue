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
use world::World;

/// Main engine
pub struct Engine {
    // world: Box<World>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            // renderer: renderer,
            // config_manager: ConfigManager::new(),
            // current_map: None,
            // entities: HashMap::new(),
            // world: Box::new(World::new())
        }
    }
    // POST /maps/generate

    

    // pub fn get_world(&self) -> &World {
    //     &self.world
    // }

    // pub fn get_mut_world(&mut self) -> &mut World {
    //     &mut self.world
    // }
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