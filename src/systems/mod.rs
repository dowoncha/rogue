use rand::{Rng, thread_rng};

use std::collections::{VecDeque};
use std::cell::RefCell;

use components::{Component, ComponentType, self};
use entities::*;

#[macro_use]
pub mod macros {
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
}

mod chronos_system;
pub use self::chronos_system::Chronos;
mod collide_system;
pub use self::collide_system::CollisionSystem;
mod render_system;
pub use self::render_system::RenderSystem;
mod input_system;
pub use self::input_system::InputSystem;

pub trait System {
    fn mount(&mut self, _: &mut EntityManager) { }
    fn process(&self, _: &mut EntityManager) {}

    fn process_mut(&mut self, _: &mut EntityManager) {}

    fn unmount(&mut self, _: &mut EntityManager) { }

    fn on_add_component(&mut self, _: Entity, _: ComponentType) {}
}

pub struct SystemManager {
    systems: Vec<Box<dyn System>>
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn mount(&mut self, em: &mut EntityManager) {
        for system in &mut self.systems {
            system.mount(em);
        }
    }

    pub fn register_system<S: 'static + Sized + System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn process_systems(&self, em: &mut EntityManager) {
        for system in &self.systems {
            system.process(em);
        }
    }

    pub fn unmount(&mut self, em: &mut EntityManager) {
        for system in &mut self.systems {
            system.unmount(em);
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
        // Get all entities that have a pickup component
        let pickup_entities = em.get_entities_with_components(components::Pickup::get_component_type());

        let ownable_entities = em.get_entities_with_components(components::Ownable::get_component_type());
        let ownable_positions = ownable_entities.iter().map(|entity| get_component!(em, *entity, components::Position));

        for entity in pickup_entities {
            let pickup_position = get_component!(em, entity, components::Position);

            // if let Some((item, item_position)) = ownable_entities.iter().find(|(item, item_position)| item_position == pickup_position) {
            // }
        }

        // Check if entity has same position as another entity that can be owned

        // Add the entity to the pickup entity's inventory

        // Preprocess
        // check if input was to pickup
        // let input_entities = em.get_entities_with_components(components::Input::get_component_type());

        // for entity in input_entities {
        //     let input = get_component!(em, entity, components::Input).unwrap();

        //     match input.input {
        //         101 => {
        //             let position = get_component!(em, entity, components::Position).unwrap();
        //             // E, Pickup action
        //             // check if there is an item at the input entitiy's position
        //             // If there is then add a Pickup component to the item
        //             let item_entities = em.get_entities_with_components(components::Item::get_component_type());

        //             let mut target = None;

        //             for item_entity in item_entities {
        //                 if let Some(item_position) = get_component!(em, entity, components::Position) {
        //                     if item_position == position {
        //                         target = Some(item_entity);
        //                         // remove position component from entity
        //                     }
        //                 }
        //             }

        //             // Add the item's template to the entity's inventory
        //             if let Some(item) = target {
        //                 let inventory = get_component!(mut, em, entity, components::Inventory).unwrap();
        //                 inventory.add_item(item);

        //                 em.remove_component(item, components::Position::get_component_type());
        //             }
        //         }
        //         _ => {}
        //     }
        // }
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
            let position = get_component!(em, entity, components::Position);

            if position.is_none() {
                continue;
            }

            let position = position.unwrap().clone();

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
        let entities = self.entities.borrow();
        let current_turn_entity = entities.front();

        if current_turn_entity.is_none() {
            return;
        }

        let current_turn_entity = *current_turn_entity.unwrap();

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