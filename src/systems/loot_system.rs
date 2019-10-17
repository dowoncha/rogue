use super::{System};

use entities::EntityManager;
use components::{Component, self};

/// Generate loot entity for all dead entities
#[derive(Debug)]
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