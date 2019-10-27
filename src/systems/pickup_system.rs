use super::System;
use crate::entities::EntityManager;
use crate::components::{Component, self};

#[derive(Debug)]
pub struct PickupSystem;

impl System for PickupSystem {
    fn process(&self, em: &mut EntityManager) {
        // Get all entities that have a pickup component
        let pickup_entities = em.get_entities_with_components(components::Pickup::get_component_type());

        let ownable_entities = em.get_entities_with_components(components::Ownable::get_component_type());
        let ownable_positions = ownable_entities.iter().map(|entity| get_component!(em, *entity, components::Position));

        for entity in pickup_entities {
            let pickup_position = get_component!(em, entity, components::Position);

            // if let Some((item, item_position)) = ownable_entities.iter().find(|(item, item_position)| item_position == pickup_position) {
            // }
        }

        // Check if entity has same position as another entity that can be owned

        // Add the entity to the pickup entity's inventory

        // Preprocess
        // check if input was to pickup
        // let input_entities = em.get_entities_with_components(components::Input::get_component_type());

        // for entity in input_entities {
        //     let input = get_component!(em, entity, components::Input).unwrap();

        //     match input.input {
        //         101 => {
        //             let position = get_component!(em, entity, components::Position).unwrap();
        //             // E, Pickup action
        //             // check if there is an item at the input entitiy's position
        //             // If there is then add a Pickup component to the item
        //             let item_entities = em.get_entities_with_components(components::Item::get_component_type());

        //             let mut target = None;

        //             for item_entity in item_entities {
        //                 if let Some(item_position) = get_component!(em, entity, components::Position) {
        //                     if item_position == position {
        //                         target = Some(item_entity);
        //                         // remove position component from entity
        //                     }
        //                 }
        //             }

        //             // Add the item's template to the entity's inventory
        //             if let Some(item) = target {
        //                 let inventory = get_component!(mut, em, entity, components::Inventory).unwrap();
        //                 inventory.add_item(item);

        //                 em.remove_component(item, components::Position::get_component_type());
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }
}