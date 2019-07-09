use ncurses as nc;

use uuid::Uuid;

use std::ops::Deref;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::collections::HashMap;

// use action::{Direction, Action, WalkAction};
// use world::World;
use renderer::ColorPair;
use console::Color;

mod player;
pub use self::player::create_player;

pub struct EntityManager {
    entities: RefCell<HashMap<String, Entity>>
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(HashMap::new())
        }
    }

    pub fn create_entity(&self) -> String {
        // Create an empty container and return the id
        let uuid = Uuid::new_v4();
        // let id: &str = uuid.to_simple().into();
        let id = String::new();

        let entity = Entity::new(&id);

        self.entities.borrow_mut().insert(id.clone(), entity);

        id
    }
}

pub type Entities = HashMap<String, Box<Entity>>;

pub struct Entity {
    id: String,
    name: String,
}

impl Entity {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: String::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub enum InputType {
    Move(i32, i32),
    Pickup,
    ShowInventory,
    Exit
    // DropInvetnryo,
    // EquipInventory
}

pub fn handle_key(key: i32) -> Option<InputType> {
    match key {
        119 => {
            // 'w
            Some(InputType::Move(0, -1))
        }
        115 => {
            // 's'
            Some(InputType::Move(0, 1))
        }
        100 => {
            // 'd'
            Some(InputType::Move(1, 0))
        }
        97 => {
            // 'a'
            Some(InputType::Move(-1, 0))
        }
        113 => {
            // 'q'
            Some(InputType::Exit)
        }
        _ => { None }
    }
}

#[derive(Clone)]
pub struct Breed {
    pub name:  String,
    pub glyph: char,
    pub color: ColorPair,
    pub max_health: i32,
    // attack: Attack
    // moves: Vec<Box<Use>>
    // flags: Set<String>,
    // loot
}