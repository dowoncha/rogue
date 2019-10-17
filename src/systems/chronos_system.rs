use super::{System};

use crate::entities::*;
use crate::components::{self};

/**
 * Chronos is the time keeper
 * He has a reference to all entities
 * And allocates time to entities for actions
 */
#[derive(Debug)]
pub struct Chronos;

impl Chronos {
    pub fn new() -> Self {
        Self
    }
}

impl System for Chronos {
    fn mount(&mut self, em: &mut EntityManager) {
        // Create game time
        let gametime = em.create_entity();
        em.add_component(gametime, components::GameTime::new());
        em.set_entity_name(gametime, "GameTime");
    }

    fn process(&self, em: &mut EntityManager) {
        // Preprocess events
        // Any time a turn timer component is added,
        // Add that entity into the turn queue

        // Give all entities with speed
        // let speed_entities = em.get_entities_with_components(components::Speed::get_component_type());

        // for entity in speed_entities {
        //     let speed = {
        //         let speed = get_component!(em, entity, components::Speed).unwrap();

        //         speed.amount
        //     };

        //     if let Some(energy) = get_component!(mut, em, entity, components::Energy) {
        //         energy.amount += speed;
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::{System, Chronos, EntityManager};

    #[test]
    fn it_should_add_gametime_entity() {
        let mut chronos = Chronos::new();

        let mut em = EntityManager::new();

        chronos.mount(&mut em);

        let gametime = em.get_entity_by_name("GameTime")
            .expect("No GameTime entity found");
    }
}