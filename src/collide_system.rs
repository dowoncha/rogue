use super::{System, EntityManager, Component};

use command_system::{Command};
use components::Position;

fn is_occupied(x: i32, y: i32) -> bool {
    // 2. Loop through all position component's to see if they occupy destination
    // let positions = em.get_all_components_of_type(Position::get_component_type());
    false
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn process(&mut self, em: &mut EntityManager) {
        // Get all update component with position componen typeu
        let commands = em.get_command_queue_mut();

        let occupied = em.get_all_components_of_type(Position::get_component_type())
            .iter();
            // .map(|component| get_component!(em, component, Position));

        // Check if they are to an occupied space
        
        // Remove from the queue
    }
}

#[test]
fn it_should_remove_update_position_commands_if_blocked() {
    use components::{CommandQueue};
    use command_system::{Command};

    let mut system = CollisionSystem;
    let mut entities = EntityManager::new();

    let command_queue = entities.create_entity();
    entities.add_component(command_queue, CommandQueue::new());

    let entity = entities.create_entity();
    entities.add_component(entity, Position { x: 0, y: 0 });

    let entity2 = entities.create_entity();
    entities.add_component(entity2, Position { x: 3, y: 3 });

    // Send some commands
    // entities.get_command_queue_mut().queue.send(Command::UpdateComponent());

    system.process(&mut entities);

    let position = get_component!(entities, entity, Position);

    assert_eq!(position.x, 1);
    assert_eq!(position.y, 1);
}