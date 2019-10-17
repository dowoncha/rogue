use super::{System};
use entities::{EntityManager};
use components::{Component, self};

#[derive(Debug)]
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