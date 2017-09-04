use std::collections::HashMap;

use std::sync::Arc;
use std::cell::RefCell;

type Client = u32;
type SkillEntry = u32;

// pub fn spawn_player(world)

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

impl Character {
    pub fn new(client: &Client) -> Character {
        unimplemented!()
    }
}

pub struct Player {
    name: String,
    level: u8
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            level: 1
        }
    }
}
