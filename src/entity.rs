use ncurses as nc;

use std::rc::Rc;
use std::collections::HashMap;

use action::{Direction, Action, WalkAction};
use world::World;
use renderer::ColorPair;

trait Component {}

pub type Entities = HashMap<String, Box<dyn Entity>>;

pub trait Entity {
    fn get_x(&self) -> i32;
    fn set_x(&mut self, x: i32);
    fn get_y(&self) -> i32;
    fn set_y(&mut self, y: i32);
    fn get_glyph(&self) -> char;
    fn get_color(&self) -> ColorPair;
    fn take_turn(&self) -> Option<Box<dyn Action>>;
    fn update(&mut self) { }
    // fn add_component(&mut self, component_name: &str, component: Box<dyn Component>) {}
}

pub struct Hero {
    name: String,
    x: i32,
    y: i32,
}

impl Hero {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: 0,
            y: 0,
        }
    }
}

impl Entity for Hero {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_glyph(&self) -> char {
        '@'
    }

    fn get_color(&self) -> ColorPair {
        ColorPair::GreenBlack
    }

    fn take_turn(&self) -> Option<Box<Action>> {
        let input = nc::getch();

        debug!("Keypress {}", input);

        None
    }

    fn update(&mut self) {
        if let Some(action) = self.take_turn() {
            action.execute(self);
        }
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

pub struct Monster {
    breed: Breed,
    max_health: i32, 
    health: i32,
    x: i32,
    y: i32,
}

impl Monster {
    pub fn new(breed: Breed, x: i32, y: i32) -> Self {
        Self {
            max_health: breed.max_health,
            health: breed.max_health,
            breed: breed,
            x: x,
            y: y,
        }
    }
}

impl Entity for Monster {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_glyph(&self) -> char {
        self.breed.glyph
    }

    fn get_color(&self) -> ColorPair {
        self.breed.color
    }

    fn take_turn(&self) -> Option<Box<Action>> {
        None
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

