use super::{Entity, ComponentType, System, Component, EntityManager};

use components::{Position, CommandQueue};

#[derive(Clone, Debug)]
pub enum Command {
    UpdateComponent(Entity, ComponentType, String)
}

pub struct CommandSystem { }

impl CommandSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn mount(&mut self, em: &mut EntityManager) {
        let entity = em.create_entity();
        em.add_component(entity, CommandQueue::new());
    }

    fn cleanup(&self, em: &mut EntityManager, num: usize) {
        // Remove consumed events from queue
    }
}

impl System for CommandSystem {
    fn process(&mut self, em: &mut EntityManager) {
        // Take a copy of the current command queue and don't process any new ones
        // let commands = em.get_command_queue_mut();
        // let mut commands_queue = commands.queue.clone();

        // let num_commands = commands_queue.len();

        // for command in commands_queue.drain(..) {
        //     // Record the command
        //     info!("{:?}", command);

        //     match command {
        //         Command::UpdateComponent(entity, component_type, component) => {
        //             if component_type == Position::get_component_type() {
        //                 let old = get_component!(mut, em, entity, Position);
        //                 *old = serde_json::from_str::<Position>(&component).unwrap();
        //             }

        //             // let new_component: Position = serde_json::from_str(&component).unwrap();l
        //             // self.update_component(em, component_type, new_component);
        //         }
        //         _ => {}
        //     }
        // }

        // self.cleanup(em, num_commands);
    }
}

#[test]
fn it_should_handle_update_component_command() {
    let mut command_system = CommandSystem { };

    let mut em = EntityManager::new();

    let command_queue = {
        let command_queue = em.create_entity();
        em.add_component(command_queue, CommandQueue::new());
        command_queue
    };

    let entity = {
        let entity = em.create_entity();
        em.add_component(entity, Position { x: 0, y: 0});

        entity
    };

    let new_position = Position { x: 1, y: 1 };
    let new_position = serde_json::to_string(&new_position).unwrap();

    // Send an event to the queue
    {
        let commands = get_component!(mut, em, command_queue, CommandQueue);
        commands.send(Command::UpdateComponent(entity, Position::get_component_type(), new_position));

        assert_eq!(commands.queue.len(), 1);
    }

    {
        command_system.process(&mut em);
    }

    {
        let position = get_component!(em, entity, Position);
        assert_eq!(*position, Position { x: 1, y: 1});

    }

    {
        let commands = get_component!(em, command_queue, CommandQueue);
        // let commands = get_component!(em, command_queue, CommandQueue);
        assert!(commands.queue.is_empty());
    }

}