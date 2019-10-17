use super::{System};
use crate::entities::EntityManager;
use crate::components::{Component, self};

#[derive(Debug)]
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