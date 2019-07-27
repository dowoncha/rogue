use serde::{Serialize, Deserialize};

use std::any::{Any, TypeId};

use super::{Entity};

pub type ComponentType = TypeId;

#[macro_use]
mod macros {
// TODO:
// Convert into procedural derive macro
#[macro_export]
macro_rules! derive_component {
    () => {
        fn get_component_type() -> super::ComponentType {
            super::ComponentType::of::<Self>()
        }

        fn get_type(&self) -> super::ComponentType {
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
}

pub trait Component: std::fmt::Debug {
    fn get_component_type() -> ComponentType where Self: Sized;
    fn get_type(&self) -> ComponentType;
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Timed;

impl Component for Timed {
    derive_component!();
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum RenderLayer {
    Player = 1000,
    Item = 100,
    Map = 10
}

#[derive(Debug)]
pub struct Render {
    pub glyph: char,
    pub layer: RenderLayer
    // fg
    // bg
}

impl Component for Render {
    derive_component!();
}

#[derive(Debug)]
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
pub struct GameTime {
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

impl Component for GameTime {
    derive_component!();
}

#[test]
fn test_game_time_tick() {
    let game_time = GameTime::new();

    let new_time = game_time.tick();
    let new_time2 = game_time.tick();

    assert_ne!(game_time, new_time);
    assert_eq!(new_time, new_time2);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Health {
    pub health: i32,
    pub max_health: i32
}

impl Component for Health {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Damage {
    pub amount: i32,
    pub target: Entity
}

impl Component for Damage {
    derive_component!();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub name: String
}

impl Component for Name {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player;

impl Component for Player {
    derive_component!();
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    Collision(Entity)
}

impl Component for Event {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RandomWalkAi;

impl Component for RandomWalkAi {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Energy {
    pub amount: i32
}

impl Component for Energy {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Speed {
    pub amount: i32
}

impl Component for Speed {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pickup {
    target: Entity
}

impl Component for Pickup {
    derive_component!();
}

impl Command for Pickup {

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Item;

impl Component for Item {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Consumable;

impl Component for Consumable {
    derive_component!();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    items: Vec<Entity>
}

impl Inventory {
    pub fn add_item(&mut self, item: Entity) {
        self.items.push(item);
    }
}

impl Component for Inventory {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Turn;

impl Component for Turn {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Prototype {
    pub prototype: Entity
}

impl Component for Prototype {
    derive_component!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Attributes {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32
}

impl Component for Attributes {
    derive_component!();
}

trait Command: Component {
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ownable;

impl Component for Ownable {
    derive_component!();
}

#[derive(Debug)]
pub struct ActionQueue {
    queue: Vec<String>,
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new()
        }
    }
}

impl Component for ActionQueue {
    derive_component!();
}

pub struct Action<T: Executable> {
    inner: T
}

impl<T> Action<T> 
    where T: Executable
{
    pub fn execute(&self, entity: Entity, components: Vec<&mut Box<dyn Component>>) {
        self.inner.execute(entity, components);
    }
}

pub trait Executable {
    fn execute(&self, entity: Entity, components: Vec<&mut Box<dyn Component>>);
}

// pub struct MoveAction {
//     dx: i32,
//     dy: i32
// }

// impl Executable for MoveAction {
//     fn execute(&self, entity: Entity, components: Vec<&mut Box<dyn Component>>) {
//         // if let Some(position) = components.iter().find(|component| component == Position::get_component_type()).unwrap() {
//         //     position.x += self.dx;
//         //     position.y += self.dy;
//         // }
//     }
// }

// #[test]
// fn it_should_move_entitys_position() {
//     let entity = Entity {
//         id: 0
//     };

//     let mut position: Box<dyn Component> = Box::new(Position {
//         x: 99,
//         y: 99
//     });

//     let action = MoveAction {
//         dx: 1, 
//         dy: 1
//     };

//     action.execute(entity, vec![&mut position]);

//     assert_eq!(position.as_any().downcast_ref::<Position>().unwrap().x, 100);
//     assert_eq!(position.as_any().downcast_ref::<Position>().unwrap().y, 100);
// }