use super::{System};
use components::{Component, self};
use entities::*;

#[derive(Debug)]
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