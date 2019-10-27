use super::{System};
use crate::entities::*;
use crate::components::{Component, self};

/**
 * Reads Input components and check if they have any input commands
 * If so then set entity's walk component
 */
#[derive(Debug)]
pub struct WalkSystem;

impl System for WalkSystem {
    fn process(&self, em: &mut EntityManager) {
        debug!("WalkSystem ----- Processing");
        // Get all entities with input components,
        let input_entities = em.get_entities_with_components(components::Input::get_component_type());

        // Get their position components
        for entity in input_entities {
            let input_component = get_component!(em, entity, components::Input).unwrap();
            
            let (dx, dy) = match input_component.input {
                119 => (0, -1),             // w
                100 => (1, 0),              // d
                115 => (0, 1),                    // s
                97 => (-1, 0),         // a
                _ => (0, 0),
            };

            // Check if there are any walk commands

            debug!("Walking {:?}, ({}, {})", entity, dx, dy);

            if let Some(walk) = get_component!(mut, em, entity, components::Walk) {
                walk.dx = dx;
                walk.dy = dy;
            }
        }
    }
}