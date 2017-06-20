use std::sync::Arc;
use std::cell::RefCell;

type Client = u32;
type SkillEntry = u32;

// pub fn spawn_player(world)

pub struct Character {
    id: u32,
    level: u32,
    attr_str: u32,
    attr_dex: u32,
    attr_luk: u32,
    attr_int: u32,
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