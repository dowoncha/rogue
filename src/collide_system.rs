use super::{System, EntityManager, Component};

use command_system::{Command};
use components::{self, Position, Collidable};
use std::collections::{VecDeque};

pub struct CollisionSystem;

impl CollisionSystem {
    fn get_occupied_spaces(&self, em: &EntityManager) -> Vec<(i32, i32)> {
        em.get_entities_with_components(Collidable::get_component_type())
            .iter()
            .map(|entity| get_component!(em, *entity, Position))
            .map(|position| (position.x, position.y))
            .collect()
    }

    fn remove_occupied_update_position_commands(&mut self, em: &mut EntityManager) {
        // Get all update component with position componen typeu
        let occupied: Vec<(i32, i32)> = self.get_occupied_spaces(em);

        let commands = em.get_command_queue_mut();

        commands.queue = commands.queue
            .iter()
            .filter(|command| {
                match command {
                    Command::UpdateComponent(_, _, serialized) => {
                        if let Ok(position) = serde_json::from_str::<Position>(&serialized) {
                            return occupied.iter().find(|p| p.0 == position.x && p.1 == position.y).is_none();
                        } else {
                            return false;
                        }
                    },
                    _ => true
                }
            })
            .cloned()
            .collect();
    }
}

impl System for CollisionSystem {
    fn process(&mut self, em: &mut EntityManager) {
        //  Check whether entitiy's walk command moves them into an occupied space
        //  If the space is occupied, 
        //
        //  Flag the space is occupied
        let walk_entities = em.get_entities_with_components(components::Walk::get_component_type());

        let occupied_spaces = self.get_occupied_spaces(em);

        for entity in walk_entities {
            let position = {
                get_component!(em, entity, components::Position).clone()
            };
            let walk = get_component!(mut, em, entity, components::Walk);

            if !(walk.dx == 0 && walk.dy == 0) {
                let dest = Position {
                    x: position.x + walk.dx,
                    y: position.y + walk.dy
                };

                if occupied_spaces.iter().any(|(x, y)| dest.x == *x && dest.y == *y) {
                    debug!("Space ({}, {}) occupied", dest.x, dest.y);
                    walk.dx = 0;
                    walk.dy = 0;
                }
            }
        }
    }
}

#[test]
fn it_should_remove_update_position_commands_if_blocked() {
    use components::{CommandQueue};
    use command_system::{Command, CommandSystem};

    let mut system = CollisionSystem;
    let mut command_system = CommandSystem::new();
    let mut entities = EntityManager::new();

    let command_queue = entities.create_entity();
    entities.add_component(command_queue, CommandQueue::new());

    let entity = entities.create_entity();
    entities.add_component(entity, Position { x: 0, y: 0 });

    let entity2 = entities.create_entity();
    entities.add_component(entity2, Position { x: 3, y: 3 });

    // Send some commands
    entities.get_command_queue_mut()
        .send(
            Command::UpdateComponent(
                entity, 
                Position::get_component_type(),
                serde_json::to_string(&Position { x: 3, y: 3 }).unwrap()));

    entities.get_command_queue_mut()
        .send(Command::UpdateComponent(entity,
            Position::get_component_type(),
            serde_json::to_string(&Position { x: 1, y: 1 }).unwrap()));

    system.process(&mut entities);

    command_system.process(&mut entities);

    let position = get_component!(entities, entity, Position);

    assert_eq!(position.x, 0);
    assert_eq!(position.y, 0);
}