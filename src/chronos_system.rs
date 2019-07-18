use std::collections::VecDeque;

use super::{EntityManager, Component, Entity, System};
use components;

/**
 * Chronos is the time keeper
 * He has a reference to all entities
 * And allocates time to entities for actions
 */
pub struct Chronos {
    event_receiver: std::sync::mpsc::Receiver<String>,
    event_sender: std::sync::mpsc::Sender<String>,
    // turns: VecDeque<Entity>
}

impl Chronos {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            event_sender: sender,
            event_receiver: receiver,
        }
    }

    // fn dependencies() -> [ComponentType] {

    // }
}

impl System for Chronos {
    fn mount(&mut self, em: &mut EntityManager) {
        em.subscribe(self.event_sender.clone());
    }

    fn process(&self, em: &mut EntityManager) {
        // Preprocess events
        // Any time a turn timer component is added,
        // Add that entity into the turn queue

        // Give all entities with speed
        let speed_entities = em.get_entities_with_components(components::Speed::get_component_type());

        for entity in speed_entities {
            let speed = {
                let speed = get_component!(em, entity, components::Speed).unwrap();

                speed.amount
            };

            if let Some(energy) = get_component!(mut, em, entity, components::Energy) {
                energy.amount += speed;
            }
        }
    }
}