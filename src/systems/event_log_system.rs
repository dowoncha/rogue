use super::{System};
use entities::{EntityManager};
use components::{Component, self};

#[derive(Debug)]
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