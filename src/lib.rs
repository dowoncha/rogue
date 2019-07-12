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

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub mod components;


pub type ComponentType = TypeId;
pub type ObjectType = TypeId;

pub type Entity = i32;

pub trait Component {
    fn get_component_type() -> ComponentType where Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct EntityManager {
    entities: Vec<Entity>,
    component_data_tables: HashMap<TypeId, HashMap<Entity, Box<dyn Component>>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            component_data_tables: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let index = self.entities.len() as Entity;
        self.entities.push(index);
        index
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T ) 
        where T: 'static + Component 
    {
        let component_type = <T as Component>::get_component_type();

        // Check if table exists, create if it doesn't
        if !self
            .component_data_tables
            // .contains_key(&component.get_object_type())
            .contains_key(&component_type)
        {
            self.component_data_tables
                .insert(component_type, HashMap::new());
        }

        // Get the component's table
        let table = self
            .component_data_tables
            .get_mut(&component_type)
            .unwrap();

        table.insert(entity, Box::new(component));
    }

    pub fn get_component(
        &self,
        entity: Entity,
        component_type: ComponentType
    ) -> Option<&Box<dyn Component>> {
        // Get the table
        if let Some(table) = self.component_data_tables.get(&component_type) {
            return table.get(&entity);
        }

        None
    }

    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Box<dyn Component>> {
        if let Some(table) = self.component_data_tables.get_mut(&component_type) {
            return table.get_mut(&entity);
        }

        None
    }

    pub fn get_entities_with_components(&self, component_type: TypeId) -> Vec<Entity> {
        use std::iter::FromIterator;

        match self.component_data_tables.get(&component_type) {
            Some(table) => Vec::from_iter(table.keys().map(|entity| *entity)),
            None => vec![]
        }

        // self.component_data_tables.get(&component_type).ok_or()

        // let iter = table.values().map(Box::as_ref);
        // Vec::new().into_iter()
    }

    // pub fn get_all_components_of_type(
    //     &self,
    //     component_type: ComponentType,
    // ) -> impl Iterator<Item = &impl Component> {
        

    //     match self.component_data_tables.get(&component_type) {
    //         Some(table) => table.values().into_iter(),
    //         None => Vec::new().into_iter(),
        
    // }
}

#[cfg(test)]
mod entity_manager_tests {
    use components::{TestComponent};
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

        let component: Option<&Box<dyn Component>> =
            entity_manager.get_component(entity, TestComponent::get_component_type());
        assert!(component.is_none());

        entity_manager.add_component(entity, test_component);

        let component: Option<&Box<dyn Component>> =
            entity_manager.get_component(entity, TestComponent::get_component_type());

        assert!(component.is_some());
    }

    #[test]
    fn test_get_entities_with_components() {
        let mut em = EntityManager::new();

        let entity = em.create_entity();
        let component = TestComponent;

        let entities = em.get_entities_with_components(TestComponent::get_component_type());

        assert_eq!(entities.len(), 0);

        em.add_component(entity, component);

        let entities = em.get_entities_with_components(TestComponent::get_component_type());

        assert_eq!(entities.len(), 1);
    }
}

mod input_system;

pub use input_system::{System, InputSystem};

// mod console;
// mod components;
// // mod action;
// mod entities;
// mod gen_map_1;
// mod command_manager;
// mod config_manager;
// mod game_state;
pub mod file_logger;
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
