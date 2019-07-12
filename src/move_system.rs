use super::{System, Entity, Component, EntityManager};

use input_system::Key;
use event_system::GameEvent;
use command_system::Command;
use components::{Input, Position};

pub struct MovementSystem {
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_move_input(&self, key: Key, input_entity: Entity, em: &EntityManager) -> Option<Command> {
        let (dx, dy) = match key {
            Key::w => (0, -1),
            Key::s => (0, 1),
            Key::a => (-1, 0),
            Key::d => (1, 0),
            _ => { return None; }
        };

        let position = get_component!(em, input_entity, Position);
        let new_position = Position { x: position.x + dx, y: position.y + dy};
        // Send move command to all entities with input component
        let new_position = serde_json::to_string(&new_position).unwrap();
        Some(Command::UpdateComponent(input_entity, Position::get_component_type(), new_position))
    }
}

impl System for MovementSystem {
    fn process(&mut self, em: &mut EntityManager) {
        // Two ways
        // 1. Input -> Event -> Movement -> Command
        // 2. Input -> Event -> Command -> Movement
        // 3. Input -> Command -> Movement
        // 1 is cleaner because movement would handle the is walkable checks
        // Handles all input events and sends the corresponding command

        // 1. Get the event queue
        // 2. Get all input events
        let events = em.get_event_queue();

        let mut commands = vec![];

        // TODO: filter out all non input game events
        events.queue.iter().for_each(|event| {
            match event {
                GameEvent::Input(key) => {
                    let input_entities = em.get_entities_with_components(Input::get_component_type());

                    for input_entity in input_entities {
                        if let Some(command) = self.handle_move_input(*key, input_entity, em) {
                            commands.push(command);
                        }
                    }
                }
                _ => {}
            }
        });

        let commands_queue = em.get_command_queue_mut();

        for command in commands {
            commands_queue.send(command);
        }
    }
}