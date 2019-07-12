use serde::{Serialize, Deserialize};

use std::any::{Any, TypeId};
use std::collections::VecDeque;

use event_system::{GameEvent};
use command_system::{Command};

pub type ComponentType = TypeId;

// TODO:
// Convert into procedural derive macro
#[macro_export]
macro_rules! derive_component {
    () => {
        fn get_component_type() -> super::ComponentType {
            super::ComponentType::of::<Self>()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }
}

pub trait Component {
    fn get_component_type() -> ComponentType where Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    derive_component!();
}

pub struct TestComponent;

impl Component for TestComponent {
    derive_component!();
}

pub struct Input {
}

impl Input {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Component for Input {
    derive_component!();
}

pub struct Timed;

impl Component for Timed {
    derive_component!();
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum RenderLayer {
    Player = 1000,
    Map = 10
}

pub struct Render {
    pub glyph: char,
    pub layer: RenderLayer
    // fg
    // bg
}

impl Component for Render {
    derive_component!();
}

pub struct Collidable;

impl Collidable {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Collidable {
    derive_component!();
}

pub struct EventQueue {
    pub queue: VecDeque<GameEvent>
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    pub fn send(&mut self, event: GameEvent) {
        self.queue.push_back(event);
    }
}

impl Component for EventQueue {
    derive_component!();
}

pub struct CommandQueue {
    pub queue: VecDeque<Command>
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    pub fn send(&mut self, command: Command) {
        self.queue.push_back(command);
    }
}

impl Component for CommandQueue {
    derive_component!();
}