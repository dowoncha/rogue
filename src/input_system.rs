use ncurses as nc;

use super::{Component, EntityManager};
use components::{Position, Input};

pub trait System {
    fn mount(&mut self);
    fn process(&mut self, entity_manager: &mut EntityManager);
}

pub struct InputSystem {
    event_sender: std::sync::mpsc::Sender<i32>,
    event_receiver: std::sync::mpsc::Receiver<i32>,
    join_handle: Option<std::thread::JoinHandle<()>>
}

impl InputSystem {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            event_sender: sender,
            event_receiver: receiver,
            join_handle: None
        }
    }

    pub fn get_thread_handle(&self) -> Option<&std::thread::JoinHandle<()>> {
        self.join_handle.as_ref()
    }
}

impl System for InputSystem {
    fn mount(&mut self) {
        let handle = start_input_thread(self.event_sender.clone());

        self.join_handle = Some(handle);
    }
    
    fn process(&mut self, entity_manager: &mut EntityManager) {
        // Check for any key events
        // Get all entities with input component
        let input_entities = entity_manager.get_entities_with_components(Input::get_component_type());

        // Move all entities
        if let Ok(input) = self.event_receiver.try_recv() {
            if let Some(input_event) = handle_input(input) {
                match input_event {
                    InputEvent::MovePlayer(dx, dy) => {
                        // Check if tile is walkable

                        // Check if it is occupied
                        for input_entity in input_entities {
                            let component = entity_manager.get_component_mut(input_entity, Position::get_component_type())
                                .expect("Entity does not have a position component");
                            let position: &mut Position = component.as_any_mut().downcast_mut::<Position>()
                                .expect("Could not downgrade component into position");

                            position.x += dx;
                            position.y += dy;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Drop for InputSystem {
    
    fn drop(&mut self) {
        // if let Some(handle) = &self.join_handle {
        //     handle.join().unwrap();
        // }
    }
}

#[derive(Debug, PartialEq)]
enum InputEvent {
    MovePlayer(i32, i32),
    Quit,
}

fn start_input_thread(input_listener: std::sync::mpsc::Sender<i32>) -> std::thread::JoinHandle<()> {
    use std::thread;

    let handle = thread::spawn(move || {
        loop {
            let input = nc::getch();

            input_listener.send(input).unwrap();
        }
    });

    handle
}

fn handle_input(input: i32) -> Option<InputEvent> {
    match input {
        119 => {
            //'w'
            Some(InputEvent::MovePlayer(0, -1))
        }
        100 => Some(InputEvent::MovePlayer(1, 0)),
        115 => Some(InputEvent::MovePlayer(0, 1)),
        97 => Some(InputEvent::MovePlayer(-1, 0)),
        113 => Some(InputEvent::Quit),
        _ => None,
    }
}

#[cfg(test)]
mod input_system_tests {
    use super::*;

    #[test]
    fn it_should_process_input() {
        let mut em = EntityManager::new();

        let entity = em.create_entity();

        em.add_component(entity, Position { x: 0, y: 0});
        em.add_component(entity, Input);

        let mut input_system = InputSystem::new(); 

        input_system.process(&mut em);

        let component = em.get_component_mut(entity, Position::get_component_type()).unwrap();
        let position = component.as_any_mut().downcast_mut::<Position>().unwrap();
    }

    #[test]
    fn it_should_create_input_thread_on_mount() {
        let mut input_system = InputSystem::new();

        input_system.mount();

        let input_thread_handle = input_system.get_thread_handle();

        assert!(input_thread_handle.is_some());
    }

    #[test]
    fn test_controls() {
        let event = handle_input(119).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(0, -1));

        //' d'
        let event = handle_input(100).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(1, 0));

        // assert!(player.x == 101 && player.y == 99);
        // 's'
        let event = handle_input(115).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(0, 1));

        // 'a'
        let event = handle_input(97).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(-1, 0));

        // 'q'
        let event = handle_input(113).unwrap();
        assert_eq!(event, InputEvent::Quit);
    }
}