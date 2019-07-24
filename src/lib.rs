#![crate_name = "rogue"]
#![crate_type = "lib"]
#![feature(duration_float)]
#![feature(option_flattening)]
#![feature(vec_remove_item)]
#![feature(let_chains)]
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

use rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cell::RefCell;

#[macro_use]
pub mod components;
pub use components::{Component, ComponentType};

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

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct Entity {
    id: i32,
}

impl Entity {
    fn new(id: i32) -> Self {
        Self {
            id: id,
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;

        Ok(())
    }
}

pub trait System {
    fn mount(&mut self, _: &mut EntityManager) { }
    fn process(&self, entity_manager: &mut EntityManager) {}

    fn process_mut(&mut self, entity_manager: &mut EntityManager) {}

    fn unmount(&mut self, _: &mut EntityManager) { }

    fn on_add_component(&mut self, entity: Entity, component_type: ComponentType) {}
}

pub struct EntityManager {
    entities: Vec<Entity>,
    component_data_tables: HashMap<ComponentType, HashMap<Entity, Box<dyn Component>>>,
    listeners: Vec<std::sync::mpsc::Sender<String>>
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            component_data_tables: HashMap::new(),
            listeners: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let index = self.entities.len() as i32;
        let entity = Entity::new(index);
        self.entities.push(Entity::new(index));

        entity
    }

    pub fn extend(&mut self, prototype: Entity, child: Entity) {
        self.add_component(child, components::Prototype { prototype: prototype });
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
            let _ = self.component_data_tables
                .insert(component_type, HashMap::new());
        }

        // Get the component's table
        let table = self
            .component_data_tables
            .get_mut(&component_type)
            .unwrap();

        let _ = table.insert(entity, Box::new(component));

        // em.listen()
        // listeners.notify("add_component entity, component_type ")
    }

    fn get_prototype(&self, entity: Entity) -> Option<Entity> {
        get_component!(self, entity, components::Prototype).map(|prototype| prototype.prototype)
    }

    pub fn get_component(
        &self,
        entity: Entity,
        component_type: ComponentType
    ) -> Option<&Box<dyn Component>> {
        // Get the table
        if let Some(table) = self.component_data_tables.get(&component_type) {
            return table.get(&entity);
        } else if let Some(prototype) = self.get_prototype(entity) {
            // Check the prototype
            self.get_component(prototype, component_type)
        } else {
            // Check for key not present tag

            None
        }
    }

    // pub fn get_generic_component<T: Component>(&self, entity: Entity) -> Option<&T> {
    //     let component = self.get_component(entity, T::get_component_type());
    //     component.map(|c| c.as_ref())
    // }

    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Box<dyn Component>> {
        let mut component = None;
        if let Some(table) = self.component_data_tables.get_mut(&component_type) {
            component = table.get_mut(&entity);
        }
        // } else if let Some(prototype) = self.get_prototype(entity) {
            // self.get_component_mut(prototype, component_type)
        component
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

    pub fn subscribe(&mut self, listener: std::sync::mpsc::Sender<String>) {
        self.listeners.push(listener);
    }
}

impl std::fmt::Debug for EntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entities")?;

        for entity in &self.entities {
            writeln!(f, "{:?}", entity)?;
        }

        Ok(())
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

pub struct SystemManager<'em> {
    entity_manager: &'em mut EntityManager,
    systems: Vec<Box<dyn System>>
}

impl<'em> SystemManager<'em> {
    pub fn new(entity_manager: &'em mut EntityManager) -> Self {
        Self {
            entity_manager: entity_manager,
            systems: Vec::new()
        }
    }

    pub fn mount(&mut self) {
        for system in &mut self.systems {
            system.mount(&mut self.entity_manager);
        }
    }

    pub fn register_system<S: 'static + Sized + System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self) {
        loop {
            self.process_systems(); 
        }
    }

    pub fn process_systems(&mut self) {
        for system in &self.systems {
            system.process(self.entity_manager);
        }
    }

    pub fn unmount(&mut self) {
        for system in &mut self.systems {
            system.unmount(&mut self.entity_manager);
        }
    }
}

/**
 * Reads Input components and check if they have any input commands
 * If so then set entity's walk component
 */
pub struct WalkSystem;

impl System for WalkSystem {
    fn process(&self, em: &mut EntityManager) {
        debug!("WalkSystem ----- Processing");
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

            // Check if there are any walk commands

            debug!("Walking {:?}, ({}, {})", entity, dx, dy);

            if let Some(walk) = get_component!(mut, em, entity, components::Walk) {
                walk.dx = dx;
                walk.dy = dy;
            }
        }
    }
}

pub struct PickupSystem;

impl System for PickupSystem {
    fn process(&self, em: &mut EntityManager) {
        // Preprocess
        // check if input was to pickup
        let input_entities = em.get_entities_with_components(components::Input::get_component_type());

        for entity in input_entities {
            let input = get_component!(em, entity, components::Input).unwrap();

            match input.input {
                101 => {
                    let position = get_component!(em, entity, components::Position).unwrap();
                    // E, Pickup action
                    // check if there is an item at the input entitiy's position
                    // If there is then add a Pickup component to the item
                    let item_entities = em.get_entities_with_components(components::Item::get_component_type());

                    let mut target = None;

                    for item_entity in item_entities {
                        if let Some(item_position) = get_component!(em, entity, components::Position) {
                            if item_position == position {
                                target = Some(item_entity);
                                // remove position component from entity
                            }
                        }
                    }

                    // Add the item's template to the entity's inventory
                    if let Some(item) = target {
                        let inventory = get_component!(mut, em, entity, components::Inventory).unwrap();
                        inventory.add_item(item);

                        em.remove_component(item, components::Position::get_component_type());
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct MoveSystem;

impl System for MoveSystem {
    fn process(&self, em: &mut EntityManager) {
        let walk_entities = em.get_entities_with_components(components::Walk::get_component_type());

        for entity in walk_entities {
            debug!("Moving entity, {}", entity);
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

pub struct AiSystem;

impl System for AiSystem {
    fn process(&self, em: &mut EntityManager) {
        // Whose turn is it?

        // Does the entity have a script

        // run the script
    }
}

pub struct AttackSystem;

impl System for AttackSystem {
    fn process(&self, em: &mut EntityManager) {
        // Get all entities with an attack component on them
        // Get collision components
        // Check if health component exists
        // Add damage component
        let entities = em.get_entities_with_components(components::Event::get_component_type());

        for entity in entities {
            if let Some(event) = get_component!(em, entity, components::Event) {
                if let components::Event::Collision(collider) = event {
                    if em.has_component(*collider, components::Health::get_component_type()) {
                        let mut rng = thread_rng();

                        let damage_amount = rng.gen_range(1, 4);
                        // let damage_entity = {
                        //     let entity = em.create_entity();
                        //     entity
                        // };
                        // em.add_component(damage_entity, components::Damage { amount: damage_amount, target: *collider });

                        em.add_component(*collider, components::Damage { amount: damage_amount, target: *collider });
                    }
                }
            }
        }
    }
}

pub struct DamageSystem;

impl System for DamageSystem {
    fn process(&self, em: &mut EntityManager) {

        let damage_entities = em.get_entities_with_components(components::Damage::get_component_type());

        // Apply damage if they have a health component
        for entity in damage_entities.into_iter() {
            let damage = get_component!(em, entity, components::Damage).unwrap().clone();
            let name = get_component!(em, entity, components::Name).map(|c| c.name.clone()).unwrap_or(entity.to_string());

            let mut damaged = None;

            if let Some(health) = get_component!(mut, em, entity, components::Health) {
                health.health -= damage.amount;

                em.remove_component(entity, components::Damage::get_component_type());

                damaged = Some(damage.amount);
            }

            if let Some(damaged) = damaged {
                let player = em.get_entities_with_components(components::Player::get_component_type())[0];
                if let Some(log) = get_component!(mut, em, player, components::Log) {
                    debug!("Damage System - Logging Damage");
                    log.history.push(format!("{} took {} damage.", name, damaged));
                }
            }
        }
    }
}

pub struct Reaper;

impl System for Reaper {
    fn process(&self, em: &mut EntityManager) {
        let health_entities = em.get_entities_with_components(components::Health::get_component_type());

        for entity in health_entities.into_iter() {
            let health = get_component!(em, entity, components::Health).unwrap();

            if health.health <= 0 {
                if let Some(name) = get_component!(em, entity, components::Name) {
                    let message = format!("{} has died", &name.name);
                    info!("{}", message);
                    let player = em.get_entities_with_components(components::Player::get_component_type())[0];
                    if let Some(log) = get_component!(mut, em, player, components::Log) {
                        log.history.push(message);
                    }
                }

                em.kill_entity(entity);
            }
        }
    }
}

/// Generate loot entity for all dead entities
pub struct LootSystem;

impl System for LootSystem {
    fn process(&self, em: &mut EntityManager) {
        let health_entities = em.get_entities_with_components(components::Health::get_component_type());

        for entity in health_entities {
            let health = get_component!(em, entity, components::Health).unwrap();
            let position = get_component!(em, entity, components::Position).unwrap().clone();

            if health.health <= 0 {
                let loot = em.create_entity();

                // TODO
                // Spawn entity item template

                em.add_component(loot, position);
                em.add_component(loot, components::Render { glyph: '!', layer: components::RenderLayer::Item });
                em.add_component(loot, components::Name { name: "Potion of Health".to_string() });
                em.add_component(loot, components::Item);
                em.add_component(loot, components::Consumable);
            }
        }
    }
}

pub struct EventLogSystem;

impl System for EventLogSystem {
    fn process(&self, em: &mut EntityManager) {
        let entities_with_events = em.get_entities_with_components(components::Event::get_component_type());

        for entity in entities_with_events {
            let event = get_component!(em, entity, components::Event).unwrap();
            info!("{:?}", event);
        }
    }
}

pub struct Janitor;

impl System for Janitor {
    fn process(&self, em: &mut EntityManager) {
        // Remove all events
        let entities_with_events = em.get_entities_with_components(components::Event::get_component_type());

        for entity in entities_with_events {
            em.remove_component(entity, components::Event::get_component_type());
        }
    }
}

pub struct RandomWalkAiSystem;

impl System for RandomWalkAiSystem {
    fn process(&self, em: &mut EntityManager) {
        debug!("Processing random walk ai system");
        let mut rng = thread_rng();

        let entities = em.get_entities_with_components(components::RandomWalkAi::get_component_type());

        for entity in entities {
            if let Some(walk) = get_component!(mut, em, entity, components::Walk) {
                walk.dx = rng.gen_range(-1, 2);
                walk.dy = rng.gen_range(-1, 2);
            }
        }
    }
}

pub struct TurnSystem {
    entities: RefCell<VecDeque<Entity>>
}

impl TurnSystem {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(VecDeque::new())
        }
    }
}

impl System for TurnSystem {
    fn mount(&mut self, em: &mut EntityManager) {
        // Add all entities with speed and energy
        let living_entities = em.get_entities_with_components(components::Energy::get_component_type());

        for entity in living_entities {
            self.entities.borrow_mut().push_back(entity);
        }
    }

    fn process(&self, em: &mut EntityManager) {
        // Check if current turn's entity's still has energy
        let current_turn_entity = *self.entities.borrow().front().unwrap();

        let energy = get_component!(em, current_turn_entity, components::Energy).unwrap();

        if energy.amount <= 0 {
            debug!("Current entity {} has no more energy", current_turn_entity);
            self.entities.borrow_mut().rotate_right(1);
            let new_turn_entity = *self.entities.borrow().front().unwrap();

            // Remove turn component from current entity
            // Give to entity
            em.remove_component(current_turn_entity, components::Turn::get_component_type());
            em.add_component(new_turn_entity, components::Turn);

            // Give energy to new entity
            let new_speed = {
                let speed = get_component!(em, new_turn_entity, components::Speed).unwrap();
                speed.amount
            };

            {
                let new_energy = get_component!(mut, em, new_turn_entity, components::Energy).unwrap();

                new_energy.amount += new_speed;
            }

            debug!("Added turn to new entity {:?}", new_turn_entity);
        }
    }
}

mod input_system;
mod render_system;
mod chronos_system;
// mod move_system;
mod collide_system;
pub mod map;
mod types;
pub mod monsters;
pub mod items;

pub use types::{Rect};
pub use map::{Map, MapBuilder};
pub use input_system::{InputSystem};
pub use render_system::{RenderSystem, drop_ncurses};
pub use chronos_system::{Chronos};
pub use collide_system::{CollisionSystem};

pub mod file_logger;

pub trait Subject {
    fn register(&mut self, observer: &dyn Observer);
    fn unregister(&mut self, observer: &dyn Observer);
    fn observers(&self) -> &[&dyn Observer];
    fn notify(&self, event: String) {
        for o in self.observers() {
            o.update(event.clone());
        }
    }
}

pub trait Observer {
    fn update(&self, event: String);
}