use super::System;
use components::{Component, self};
use entities::{EntityManager};

#[derive(Debug)]
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