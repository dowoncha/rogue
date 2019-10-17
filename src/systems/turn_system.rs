use super::System;
use crate::entities::{Entity, EntityManager};
use crate::components::{Component, self};

use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TurnSystem {
    entities: RefCell<VecDeque<Entity>>
}

impl TurnSystem {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(VecDeque::new())
        }
    }

    fn process_turn(&self, em: &mut EntityManager) {
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

impl System for TurnSystem {
    fn mount(&mut self, em: &mut EntityManager) {
        // Add all entities with speed and energy
        let living_entities = em.get_entities_with_components(components::Energy::get_component_type());

        for entity in living_entities {
            self.entities.borrow_mut().push_back(entity);
        }
    }

    fn process(&self, em: &mut EntityManager) {
        const turn_length: i32 = 24;

        // Get all entities with energy
        let entities_with_energy = em.get_entities_with_components(components::Energy::get_component_type());

        for entity in entities_with_energy {
            // 1. Subtract each entity's speed from it's energy
            {
                let speed = { get_component!(em, entity, components::Speed).cloned() };
                let energy = get_component!(mut, em, entity, components::Energy).unwrap();

                if let Some(speed) = speed {
                    energy.amount -= speed.amount;
                }
            }

            // 2. If energy is less than 0, give the entity a move
            {
                let energy = get_component!(mut, em, entity, components::Energy).unwrap();

                if energy.amount < 0 {
                    // Move entity to back of line
                    energy.amount += turn_length;
                }
            }
        }
    }
}