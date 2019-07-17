use serde::{Serialize, Deserialize};

use std::any::{Any, TypeId};

use super::{Entity};

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

pub struct Player;

impl Component for Player {
    derive_component!();
}

pub struct Log {
    pub history: Vec<String>
}

impl Log {
    pub fn new() -> Self {
        Self {
            history: Vec::new()
        }
    }
}

impl Component for Log {
    derive_component!();
}

#[derive(Debug)]
pub enum Event {
    Collision(Entity)
}

impl Component for Event {
    derive_component!();
}

pub struct RandomWalkAi;

impl Component for RandomWalkAi {
    derive_component!();
}

pub struct TurnTimer {
}

impl Component for TurnTimer {
    derive_component!();
}

// pub struct AttackAI;

// impl Component for AttackAi {
//     derive_component!();
// ai}