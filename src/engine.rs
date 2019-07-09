// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);

use std::cell::RefCell;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use rand::prelude::*;
use ncurses as nc;

// use action::Action;
use entities::{Entity, EntityManager};
use config_manager::ConfigManager;
use renderer::{TerminalRenderer, ColorPair};
use character::Player;
use types::{Dimension, BoxResult, Rect};
use map::{Map, MapBuilder, Cell as MapCell};
// use world::World;
use components::{ComponentManager, Component};
use systems::{SystemManager, MovementSystem};

pub mod events {
    use super::GameEvent;

    pub const MoveEntity: &str = "MoveEntity";
}

#[derive(Clone)]
pub struct GameEvent {
    pub kind: String,
    pub args: Vec<String>
}

/// Main engine
pub struct Engine {
    entity_manager: RefCell<EntityManager>,
    component_manager: RefCell<ComponentManager>,
    system_manager: RefCell<SystemManager>,
    event_manager: RefCell<EventManager>,
    event_queue: RefCell<Vec<GameEvent>>
}

impl Engine {
    pub fn new() -> Self {
        let event_manager = EventManager::new();
        let entity_manager = EntityManager::new();
        let component_manager = ComponentManager::new();
        let system_manager = SystemManager::new();

        Self {
            entity_manager: RefCell::new(entity_manager),
            component_manager: RefCell::new(component_manager),
            event_manager: RefCell::new(event_manager),
            system_manager: RefCell::new(system_manager),
            event_queue: RefCell::new(Vec::new())
        }
    }

    pub fn init(&self) {
        // Register systems
        self.system_manager.borrow_mut().register_system(Box::new(MovementSystem));

        // Register event listeners for all systems
        for system in self.system_manager.borrow().get_systems().values() {
            // Get all events the system listens to
            for event in system.events() {
                let handler: Box<dyn Fn(GameEvent)> = Box::new(|game_event| {
                    println!("{}", game_event.kind);
                });

                // self.event_manager.boregister_listener(event, handler);
            }
        }
    }

    pub fn send_event(&self, event: GameEvent) {
        self.event_queue.borrow_mut().push(event);
    }

    pub fn get_entity_component(&self, entity_id: &str, component_name: &str) -> Option<&Box<dyn Component>> {
        // self.component_manager.borrow().get_entity_component(entity_id, component_name)
        unimplemented!()
    }

    pub fn update(&self) {
        let mut event_queue = self.event_queue.borrow_mut();

        // Process events
        for game_event in event_queue.drain(..) {
            for system in self.system_manager.borrow_mut().get_mut_systems().values() {
                if let Some(includes) = system.events().iter().find(|system_event| system_event == &&&game_event.kind) {
                    system.handle_event(game_event.clone(), self);
                }
            }
        }
    }

    pub fn create_entity(&self) -> String {
        self.entity_manager.borrow_mut().create_entity()
    }

    pub fn register_component(&self, entity_id: &str, component: Box<dyn Component>) {
        // self.component_manager.borrow_mut().create_component(&entity_id, component);
    }
}

pub struct EventManager {
    events: RefCell<Vec<GameEvent>>,
    listeners: RefCell<HashMap<String, Vec<Box<dyn Fn(GameEvent)>>>>
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            listeners: RefCell::new(HashMap::new())
        }
    }

    pub fn send_event(&self, event: GameEvent) {
    }

    pub fn register_listener(&self, event_type: &str, cb: Box<dyn Fn(GameEvent)>) {
        let mut listeners = self.listeners.borrow_mut();
        let mut event_listeners = listeners.get_mut(event_type);

        match event_listeners {
            Some(event_listeners) => {
                event_listeners.push(Box::new(cb));
            }
            None => {
                listeners.insert(event_type.to_string(), vec![Box::new(cb)]);
            }
        }
    }

    pub fn process_events(&self) {
        for event in self.events.borrow_mut().drain(..) {
            let listeners = self.listeners.borrow();
            let event_listeners = listeners.get(&event.kind);

            if let Some(event_listeners) = event_listeners {
                for listener in event_listeners {
                    listener(event.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
