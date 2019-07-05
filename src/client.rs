use std::cell::RefCell;

use rand::prelude::*;

use types::{BoxResult, Rect};
use engine::Engine;
use renderer::ColorPair;
use entity::Entity;

pub struct GameClient {
    engine: RefCell<Engine>
}

impl GameClient {
    pub fn new() -> Self {
        Self {
            engine: RefCell::new(Engine::new())
        }
    }

    pub fn init(
        &self,
        args: Vec<String>
    ) -> BoxResult<()> {
        let screen_width = 80;
        let screen_height = 50;

        let mut engine = self.engine.borrow_mut();

        engine.init(
            screen_width,
            screen_height
        );

        let player = Entity::new(screen_width as i32 / 2, screen_height as i32 / 2, '@', Some(ColorPair::GreenBlack));
        // let npc = Entity::new(screen_width as i32 / 2 - 4, screen_height as i32 / 2, '@', 0x33);

        engine.register_entity("player", player);
        // engine.register_entity("npc", npc);

        let max_rooms = 20;
        let room_min_size = 5;
        let room_max_size = 20;

        let map_rooms = engine.make_map(
            max_rooms,
            room_min_size,
            room_max_size,
            screen_width, 
            screen_height
        );

        let max_monsters_per_room = 3;

        // For each room
        // Generate monsters
        // Register each monster to engine
        map_rooms.iter()
            .map(|room| self.gen_entities(&room, max_monsters_per_room))
            .flatten()
            .enumerate()
            .for_each(|(index, entity)| {
                let id = format!("monster-{}", index);
                engine.register_entity(&id, entity)
            });

        Ok(())
    }

    fn gen_entities(&self, room: &Rect, max_monsters_per_room: i32) -> Vec<Entity> {
        let mut rng = thread_rng();
        let num_monsters = rng.gen_range(0, max_monsters_per_room);

        let mut entities: Vec<Entity> = Vec::new();

        for _ in 0..num_monsters {
            let x = rng.gen_range(room.x1 + 1, room.x2 - 1);
            let y = rng.gen_range(room.y1 + 1, room.y2 - 1);

            // Check if any entity resides in selected cell
            if entities.iter().filter(|e| e.x == x && e.y == y).count() == 0 {
                let entity = Entity::new(x, y, 'T', Some(ColorPair::RedBlack));

                entities.push(entity);
            }

        }

        entities
    }

    pub fn run(&self) -> BoxResult<()>  {
        self.engine.borrow_mut().run()
    }

    pub fn load_map(&self, filename: &str) -> BoxResult<()> {
        self.engine.borrow_mut().load_map(filename)
    }
}

struct Message {
    text: String,
    // color: Color
}

struct MessageLog {
    messages: Vec<Message>
}

impl MessageLog {
    pub fn new() -> Self {
        Self {
            messages: Vec::new()
        }
    }

    pub fn add_message(&mut self, message: Message) {
        // Split the message if necessary, among multiple lines
        self.messages.push(message);
    }
}