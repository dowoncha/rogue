use std::collections::HashMap;

use std::sync::Arc;
use std::cell::RefCell;

pub struct Attributes {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8
}

struct Inventory {
    // items: HashMap<>
}

pub struct Character {
    id: u32,
    level: u32,
    attributes: Attributes,
    hp: u32,
    max_hp: u32,
    mp: u32,
    max_mp: u32,
    name: String,
    exp: Arc<RefCell<u32>>,
    gold: Arc<RefCell<u32>>,
    // skills: HashMap<Skill, SkillEntry>,
    // client: &Client,
    // buddy_list: BuddyList,
    // party: 
    // quests: HashMap<Quest>,
}

pub struct Player {
    name: String,
    level: u8,
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            level: 1,
            x: 0,
            y: 0
        }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
