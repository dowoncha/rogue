#![crate_name = "rogue"]
#![crate_type = "lib"]

#![feature(duration_float)]

#![recursion_limit = "1024"]

extern crate backtrace;

// #[macro_use]
// extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate rand;
extern crate rand_distr;

extern crate serde;
extern crate serde_json;

extern crate time;

extern crate ncurses;

extern crate uuid;

use std::collections::HashMap;

pub type Entity = i32; 

#[derive(PartialEq, Eq, Hash)]
pub enum ComponentType {
  Test = 0
}

struct TestComponent;

impl Component for TestComponent {
  fn get_component_type(&self) -> ComponentType {
    ComponentType::Test
  }
}

pub trait Component {
  fn get_component_type(&self) -> ComponentType;
}

pub trait System {
  fn process_one_game_tick(previous_frame_time: i128);
}

pub struct EntityManager {
  entities: Vec<Entity>,
  component_data_tables: HashMap<ComponentType, HashMap<Entity, Box<dyn Component>>>
}

impl EntityManager {
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
      component_data_tables: HashMap::new()
    }
  }

  pub fn create_entity(&mut self) -> Entity {
    let index = self.entities.len() as Entity;
    self.entities.push(index);
    index
  }

  pub fn add_component(&mut self, entity: Entity, component: impl Component + 'static) {
    // Check if table exists, create if it doesn't
    if !self.component_data_tables.contains_key(&component.get_component_type()) {
      self.component_data_tables.insert(component.get_component_type(), HashMap::new());
    }

    // Get the component's table
    let mut table = self.component_data_tables.get_mut(&component.get_component_type()).unwrap();

    table.insert(entity, Box::new(component));
  }

  pub fn get_component(&self, entity: Entity, component_type: ComponentType) -> Option<&Box<dyn Component>> {
    // Get the table
    if let Some(table) = self.component_data_tables.get(&component_type) {
      return table.get(&entity);
    }

    None
  }

  pub fn get_all_components_of_type(&self, component_type: ComponentType) -> impl Iterator<Item = &impl Component> {
    let table = self.component_data_tables.get(&component_type);

    match table {
      Some(table) => {
        table.values()
      }
      None => {
        vec![].into_iter()
      }
    }
  }
}

#[cfg(test)]
mod entity_manager_tests {
  use super::*;

  #[test]
  fn test_create_entity() {
    let mut entity_manager = EntityManager::new();

    let entity = entity_manager.create_entity();

    assert_eq!(entity, 0);
    
    let entity2 = entity_manager.create_entity();
    assert_eq!(entity2, 1);
  }

  #[test]
  fn test_add_component() {
    let mut entity_manager = EntityManager::new();
    let entity = entity_manager.create_entity();

    let test_component = TestComponent;

    let component: Option<&Box<dyn Component>> = entity_manager.get_component(entity, test_component.get_component_type());
    assert!(component.is_none());

    entity_manager.add_component(entity, TestComponent);

    let component: Option<&Box<dyn Component>> = entity_manager.get_component(entity, test_component.get_component_type());

    assert!(component.is_some());
  }
}


// mod console;
// mod components;
// // mod action;
// mod entities;
// mod gen_map_1;
// mod command_manager;
// mod config_manager;
// mod game_state;
// pub mod file_logger;
// mod types;
// mod map;
// mod character;
// mod dungeon;
// mod engine;
// mod renderer;
// mod client;
// // mod world;
// mod systems;

// pub use client::GameClient;
