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

/**
 * Keyboard input system
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
    w,
    a,
    s,
    d,
    q
}

pub struct Input {
    pub input: i32
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: 0
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

#[derive(Debug, Copy, Clone)]
pub struct Walk {
    pub dx: i32,
    pub dy: i32
}

impl Walk {
    pub fn new() -> Self {
        Self {
            dx: 0,
            dy: 0
        }
    }
}

impl Component for Walk {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct GameTime {
    pub sec: i32,
    pub min: i32,
    pub hour: i32,
    pub day: i32,
    pub year: i32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            sec: 0,
            min: 0,
            hour: 0,
            day: 0,
            year: 0,
        }
    }

    pub fn tick(&self) -> Self {
        let mut new_time = self.clone();

        new_time.sec += 1;
        if new_time.sec == 60 {
            new_time.min += 1;
        } else if new_time.min == 60 {
            new_time.hour += 1;
        } else if self.hour == 24 {
            new_time.day += 1;
        } else if self.day == 365 {
            new_time.year += 1;
        }

        new_time.sec %= 60;
        new_time.min %= 60;
        new_time.hour %= 24;
        new_time.day %= 365;

        new_time
    }
}

#[test]
fn test_game_time_tick() {
    let game_time = GameTime::new();

    let new_time = game_time.tick();
    let new_time2 = game_time.tick();

    assert_ne!(game_time, new_time);
    assert_eq!(new_time, new_time2);
}

pub struct Health {
    pub health: i32,
    pub max_health: i32
}

impl Component for Health {
    derive_component!();
}

struct Physics {

}

#[derive(Copy, Clone)]
pub struct Damage {
    pub amount: i32,
}

impl Component for Damage {
    derive_component!();
}

#[derive(Clone)]
pub struct Name {
    pub name: String
}

impl Component for Name {
    derive_component!();
}
