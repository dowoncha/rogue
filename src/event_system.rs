use super::{Component, System, EntityManager};

#[macro_use]
use components::{EventQueue};

use input_system::Key;

pub struct EventSystem {
}

impl EventSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn mount(&mut self, em: &mut EntityManager) {
        let entity = em.create_entity();
        em.add_component(entity, EventQueue::new());
    }

    pub fn cleanup(&self, em: &mut EntityManager) {
        // Remove all events
        let events = em.get_event_queue_mut();
        events.queue.drain(..);
    }
}

impl System for EventSystem {
    fn process(&mut self, em: &mut EntityManager) {
        let events = em.get_event_queue();
        for event in events.queue.iter() {
            info!("{:?}", event);
        }

        // Write events to log
        // self.cleanup(em);
    }
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    Input(Key)
}

#[test]
fn it_should_handle_update_component_event() {
    let mut es = EventSystem { };

    let mut em = EntityManager::new();

    let event_queue = {
        let event_queue = em.create_entity();
        em.add_component(event_queue, EventQueue::new());
        event_queue
    };

    // let entity = {
    //     let entity = em.create_entity();
    //     em.add_component(entity, Position { x: 0, y: 0});

    //     entity
    // };

    // // Send an event to the queue
    // let eq = get_component!(mut, em, event_queue, EventQueue);
    // let new_position = Position { x: 1, y: 1 };
    // let new_position = serde_json::to_string(&new_position).unwrap();
    // eq.send(GameEvent::UpdateComponent(entity, Position::get_component_type(), new_position));

    // assert_eq!(eq.queue.len(), 1);

    // {
    //     es.process(&mut em);
    // }

    // let component = em.get_component(entity, Position::get_component_type()).unwrap();
    // let position = component.as_any().downcast_ref::<Position>().unwrap();
    // assert_eq!(*position, Position { x: 1, y: 1});

    // let eq = get_component!(em, event_queue, EventQueue);
    // assert!(eq.queue.is_empty());

}