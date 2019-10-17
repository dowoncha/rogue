use super::{System};
use entities::EntityManager;
use components::{Component, self};

use rand::{Rng, thread_rng};

#[derive(Debug)]
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