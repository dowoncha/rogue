#![crate_name = "rogue"]
#![crate_type = "lib"]
#![feature(duration_float)]
#![feature(option_flattening)]
#![feature(vec_remove_item)]
#![recursion_limit = "1024"]

/**
 * Input -> Walk -> Collision -> Move
 */

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

#[macro_use]
pub mod components;
pub use components::{Component, ComponentType, EventQueue, CommandQueue};

#[macro_export]
macro_rules! get_component {
    ($em:expr, $entity:expr, $component:ty) => {
        {
            $em.get_component($entity, <$component>::get_component_type())
                .map(|component| component.as_any().downcast_ref::<$component>())
                .flatten()
                // .expect("No component found for entity")
        }
    };
    ($i:ident, $em: expr, $entity: expr, $component:ty) => {
        {
            $em.get_component_mut($entity, <$component>::get_component_type())
                .map(|component| component.as_any_mut().downcast_mut::<$component>())
                .flatten()
        }
    }
}

pub type Entity = i32;

pub trait System {
    fn process(&mut self, entity_manager: &mut EntityManager);
}

pub struct EntityManager {
    entities: Vec<Entity>,
    component_data_tables: HashMap<ComponentType, HashMap<Entity, Box<dyn Component>>>,
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

    pub fn get_entity_all_components(&self, entity: Entity) -> Vec<&Box<dyn Component>> {
        self.component_data_tables
            .iter()
            .filter_map(|(_, component_table)| component_table.get(&entity))
            .collect()
    }

    pub fn remove_component(&mut self, entity: Entity, component_type: ComponentType) -> Option<Box<dyn Component>> {
        self.component_data_tables
            .get_mut(&component_type)
            .map(|component_table| component_table.remove(&entity))
            .flatten()
    }

    pub fn get_entities_with_components(&self, component_type: ComponentType) -> Vec<Entity> {
        use std::iter::FromIterator;

        match self.component_data_tables.get(&component_type) {
            Some(table) => Vec::from_iter(table.keys().map(|entity| *entity)),
            None => vec![]
        }

        // self.component_data_tables.get(&component_type).ok_or()

        // let iter = table.values().map(Box::as_ref);
        // Vec::new().into_iter()
    }

    pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
        if let Some(table) = self.component_data_tables.get(&component_type) {
            return table.contains_key(&entity);
        }

        return false;
    }

    pub fn get_all_components_of_type(
        &self,
        component_type: ComponentType,
    ) -> Vec<&Box<dyn Component>> {
        use std::iter::FromIterator;

        match self.component_data_tables.get(&component_type) {
            Some(table) => Vec::from_iter(table.values().into_iter()),
            None => vec![],
        }
    }

    pub fn kill_entity(&mut self, entity: Entity) {
        for (_, table) in self.component_data_tables.iter_mut() {
            let _ = table.remove(&entity);
        }

        self.entities.remove_item(&entity).unwrap();
    }
}

impl std::fmt::Debug for EntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entities\n{:?}", self.entities)
    }
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

pub struct WalkSystem;

impl System for WalkSystem {
    fn process(&mut self, em: &mut EntityManager) {
        // Get all entities with input components,
        let input_entities = em.get_entities_with_components(components::Input::get_component_type());

        // Get their position components
        for entity in input_entities {
            let input_component = get_component!(em, entity, components::Input).unwrap();

            let (dx, dy) = match input_component.input {
                119 => (0, -1),             // w
                100 => (1, 0),              // d
                115 => (0, 1),                    // s
                97 => (-1, 0),         // a
                _ => (0, 0),
            };

            if let Some(walk) = get_component!(mut, em, entity, components::Walk) {
                walk.dx = dx;
                walk.dy = dy;
            }
        }
    }
}

pub struct MoveSystem;

impl System for MoveSystem {
    fn process(&mut self, em: &mut EntityManager) {
        let walk_entities = em.get_entities_with_components(components::Walk::get_component_type());

        for entity in walk_entities {
            let walk = {
                get_component!(em, entity, components::Walk).unwrap().clone()
            };

            if let Some(position) = get_component!(mut, em, entity, components::Position) {
                // info!("Entity position ({}, {}) - walk ({}, {})", position.x, position.y, walk.dx, walk.dy);

                if !(walk.dx == 0 && walk.dy == 0) {
                    let x = position.x + walk.dx;
                    let y = position.y + walk.dy;

                    // info!("Moving entity {} ({}, {})", entity, x, y);

                    position.x = x;
                    position.y = y;
                }
            }
        }
    }
}

#[test]
fn test_move_system_process() {
    let ms = MoveSystem;

    let mut em = EntityManager::new();

    let entity = em.create_entity();
    em.add_component(entity, components::Position { x: 50, y: 50 });
    em.add_component(entity, components::Walk { dx: 0, dy: 0 });
}

pub struct DamageSystem;

impl System for DamageSystem {
    fn process(&mut self, em: &mut EntityManager) {
        let damage_entities = em.get_entities_with_components(components::Damage::get_component_type());

        // Apply damage if they have a health component
        for entity in damage_entities.into_iter() {
            let damage = get_component!(em, entity, components::Damage).unwrap().clone();
            // let name = get_component!(em, entity, components::Name).unwrap_or(&components::Name { name: "noname".to_string()});

            // let health_component = get_component!(mut, em, entity, components::Health);
            if let Some(health) = get_component!(mut, em, entity, components::Health) {
                health.health -= damage.amount;

                // info!("{} {}/{} hp", &name.name, health.health, health.max_health);

                em.remove_component(entity, components::Damage::get_component_type());
            }
        }
    }
}

pub struct Reaper;

impl System for Reaper {
    fn process(&mut self, em: &mut EntityManager) {
        let health_entities = em.get_entities_with_components(components::Health::get_component_type());

        for entity in health_entities.into_iter() {
            let health = get_component!(em, entity, components::Health).unwrap();

            if health.health <= 0 {
                if let Some(name) = get_component!(em, entity, components::Name) {
                    info!("{} has died", &name.name);
                }

                em.kill_entity(entity);
            }
        }
    }
}

mod input_system;
mod render_system;
mod chronos_system;
// mod move_system;
pub mod command_system;
pub mod event_system;
mod collide_system;
pub mod map;
mod types;

pub use types::{Rect};
pub use map::{Map, MapBuilder};
pub use input_system::{InputSystem};
pub use render_system::{RenderSystem, drop_ncurses};
pub use chronos_system::{Chronos};
// pub use move_system::{MovementSystem};
pub use collide_system::{CollisionSystem};
// pub use command_system::{CommandSystem};

pub mod file_logger;
