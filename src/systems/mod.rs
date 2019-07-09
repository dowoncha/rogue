use std::collections::HashMap;
use std::borrow::Borrow;
use std::cell::RefCell;

use serde::{Serialize};

use components::{Component, Position};
use engine::{Engine, GameEvent, EventManager};
use engine::events::{MoveEntity};

pub struct SystemManager {
    pub systems: HashMap<String, Box<dyn System>>
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new()
        }
    }

    pub fn register_system(
        &mut self, 
        system: Box<dyn System>, 
    ) {
        let system_name = system.name().to_string();
        self.systems.insert(system_name.clone(), system);
    }

    pub fn get_systems(&self) -> &HashMap<String, Box<dyn System>> {
        &self.systems
    }

    pub fn get_mut_systems(&mut self) -> &HashMap<String, Box<dyn System>> {
        &mut self.systems
    }
}

pub trait System {
    fn name(&self) -> &str;
    fn events(&self) -> &[&str];
    fn update(&self, dt: i32);
    fn handle_event(&self, event: GameEvent, engine: &Engine);
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn name(&self) -> &'static str {
        "movement"
    }

    fn events(&self) -> &[&str] {
        static events: [&str; 1] = ["position"];

        &events
    }

    fn update(&self, dt: i32) {}

    fn handle_event(&self, event: GameEvent, engine: &Engine) {
        match event.kind.as_str() {
            MoveEntity => {
                // let entity_id = event.args.get(0).unwrap();
                // Get position and velocity components
                // let position_component = engine.get_entity_component(entity_id, "position").unwrap();
                // let position = position.downcast::<Box<Position>>();
                // let velocity: &Velocity = engine.get_entity_component(entity_id, "velocity").unwrap();

                // let new_position = Position{ x: position.x + velocity.dx, y: position.y + velocity.dy};

                // engine.send_event(GameEvent {
                    // kind: "UpdateComponent".to_string(),
                    // args: vec![entity_id.clone(), "position".to_string(), serde_json::to_string(&new_position).unwrap()]
                // });
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    struct TestSystem;

    impl TestSystem {
    }

    #[test]
    fn it_should_register_system() {
        // System listens to specific events
        let system_manager = SystemManager;
        let test_system = TestSystem;

        system_manager.register_system(test_system);
    }

    #[test]
    fn it_should_move_entity() {
        let system_manager = SystemManager;

        let movement_system = MovementSystem;
    }
}