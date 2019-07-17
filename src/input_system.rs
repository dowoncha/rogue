use ncurses as nc;

use super::{System, Component, EntityManager};
use components::{Input};

pub struct KeyboardController {
    keycode: i32,
}

impl Component for KeyboardController {
    derive_component!();
}

pub struct InputSystem {
    event_sender: std::sync::mpsc::Sender<i32>,
    event_receiver: std::sync::mpsc::Receiver<i32>,
    join_handle: Option<std::thread::JoinHandle<()>>
}

/**
 * Keyboard input system
 * Describe how the user wants to move, such as walk left, jump, attack 
 */
impl InputSystem {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            event_sender: sender,
            event_receiver: receiver,
            join_handle: None
        }
    }

    pub fn mount(&mut self) {
        let handle = start_input_thread(self.event_sender.clone());

        self.join_handle = Some(handle);
    }

    pub fn get_thread_handle(&self) -> Option<&std::thread::JoinHandle<()>> {
        self.join_handle.as_ref()
    }

    pub fn get_event_sender(&self) -> std::sync::mpsc::Sender<i32> {
        self.event_sender.clone()
    }

    fn process_input_events(&self, entity_manager: &mut EntityManager) {
        debug!("Processing input");
        // Check for any key events
        // Get all entities with input component
        let input_entities = entity_manager.get_entities_with_components(Input::get_component_type());

        debug!("Found {} entities with input", input_entities.len());
        // let input_key = nc::getch();

        // If an input event is received, notify all input components
        if let Ok(input_key) = self.event_receiver.try_recv() {
        // if input_key != 0 {
            debug!("Received input {}", input_key);

            // Move all entities
            for entity in input_entities {
                let input_component = get_component!(mut, entity_manager, entity, Input).unwrap();
                input_component.input = input_key;
            }
        } else {
            for entity in input_entities {
                let input_component = get_component!(mut, entity_manager, entity, Input).unwrap();
                input_component.input = 0;
            }
        }
    }
}

impl System for InputSystem {
    fn process(&self, entity_manager: &mut EntityManager) {
        self.process_input_events(entity_manager);
    }
}

impl Drop for InputSystem {
    fn drop(&mut self) {
        if let Some(handle) = self.join_handle.take() {
            handle.join().unwrap();
        }
    }
}

fn start_input_thread(input_listener: std::sync::mpsc::Sender<i32>) -> std::thread::JoinHandle<()> {
    use std::thread;

    let handle = thread::spawn(move || {
        loop {
            let input = nc::getch() as i32;

            debug!("Input Thread: {}", input);

            input_listener.send(input).unwrap();
        }
    });

    handle
}

// fn handle_input(input: i32) -> Option<Key> {
//     match input {
//         119 => {
//             //'w'
//             Some(Key::w)
//         }
//         100 => Some(Key::d),
//         115 => Some(Key::s),
//         97 => Some(Key::a),
//         113 => Some(Key::q),
//         _ => None,
//     }
// }

#[cfg(test)]
mod input_system_tests {
    use super::*;

    #[test]
    fn it_should_process_input() {
        let mut em = EntityManager::new();

        let entity = em.create_entity();

        // em.add_component(entity, Position { x: 0, y: 0});
        em.add_component(entity, Input::new());

        let mut input_system = InputSystem::new(); 
        // input_system.get_event_sender().send(119).unwrap();

        // input_system.process(&mut em);

        // let component = em.get_component_mut(entity, Input::get_component_type()).unwrap();
        // let input = component.as_any_mut().downcast_mut::<Input>().unwrap();

        // assert_eq!(input.events[0], InputEvent::MovePlayer(0, -1));
    }

    #[test]
    fn it_should_create_input_thread_on_mount() {
        let mut input_system = InputSystem::new();

        input_system.mount();

        let input_thread_handle = input_system.get_thread_handle();

        assert!(input_thread_handle.is_some());
    }

    // #[test]
    // fn test_controls() {
    //     let event = handle_input(119).unwrap();
    //     assert_eq!(event, InputEvent::MovePlayer(0, -1));

    //     //' d'
    //     let event = handle_input(100).unwrap();
    //     assert_eq!(event, InputEvent::MovePlayer(1, 0));

    //     // assert!(player.x == 101 && player.y == 99);
    //     // 's'
    //     let event = handle_input(115).unwrap();
    //     assert_eq!(event, InputEvent::MovePlayer(0, 1));

    //     // 'a'
    //     let event = handle_input(97).unwrap();
    //     assert_eq!(event, InputEvent::MovePlayer(-1, 0));

    //     // 'q'
    //     let event = handle_input(113).unwrap();
    //     assert_eq!(event, InputEvent::Quit);
    // }
}