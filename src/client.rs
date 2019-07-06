use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};

use rand::prelude::*;

use map::Map;
use types::{BoxResult, Rect};
use action::{Action, WalkAction};
use input_manager;
use engine::{Engine, Entities};
use renderer::{TerminalRenderer, ColorPair};
use entity::{Entity, Breed, Hero, Monster};

fn gen_entities(room: &Rect, max_monsters_per_room: i32) -> Vec<Box<dyn Entity>> {
    let mut rng = thread_rng();
    let num_monsters = rng.gen_range(0, max_monsters_per_room);

    let mut entities: Vec<Box<Entity>> = Vec::new();

    let goblin_breed = Breed {
        name: "goblin".to_string(),
        glyph: 'g',
        max_health: 10,
        color: ColorPair::GreenBlack
    };

    for _ in 0..num_monsters {
        let x = rng.gen_range(room.x1 + 1, room.x2 - 1);
        let y = rng.gen_range(room.y1 + 1, room.y2 - 1);

        // Check if any entity resides in selected cell
        if entities.iter().filter(|e| e.get_x() == x && e.get_y() == y).count() == 0 {
            let entity = Box::new(Monster::new(
                goblin_breed.clone(),
                x, y));

            entities.push(entity);
        }
    }

    entities
}

pub enum Event {
    Move(i32, i32),
    Quit
}

pub struct GameClient {
    renderer: TerminalRenderer,
    engine: Box<Engine>,
    event_sender: std::sync::mpsc::Sender<Event>,
    event_receiver: std::sync::mpsc::Receiver<Event>
}

impl GameClient {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            renderer: TerminalRenderer::new(),
            engine: Box::new(Engine::new()),
            event_sender: sender,
            event_receiver: receiver
        }
    }

    pub fn init(
        &mut self,
        args: Vec<String>
    ) -> BoxResult<()> {
        let screen_width = 80;
        let screen_height = 50;

        self.renderer.init()
            .expect("Failed to init renderer");

        // let mut engine = self.engine.borrow_mut();

        // Input manager
        // input_manager::init(self.event_sender.clone());

        let max_rooms = 20;
        let room_min_size = 5;
        let room_max_size = 20;

        let (map_rooms, map) = self.engine.make_map(
            max_rooms,
            room_min_size,
            room_max_size,
            screen_width, 
            screen_height
        );

        self.engine.get_mut_world().set_map(map);

        let mut player = Box::new(Hero::new("Rand Al'Thor"));
        let first_room_center = map_rooms[0].center();
        let player_x = first_room_center.0;
        let player_y = first_room_center.1;
        player.set_x(player_x);
        player.set_y(player_y);

        info!("Hero created in {}, {}", player_x, player_y);

        self.engine.get_mut_world().register_entity("player", player);

        let max_monsters_per_room = 3;

        // For each room
        // Generate monsters
        // Register each monster to engine
        let entities = map_rooms.iter()
            .map(|room| gen_entities(&room, max_monsters_per_room))
            .flatten()
            .enumerate();

        for (index, entity) in entities {
            let id = format!("monster-{}", index);
            self.engine.get_mut_world().register_entity(&id, entity)
        }

        Ok(())
    }

    pub fn run(&mut self) -> BoxResult<()> {
        // TODO/DECISION
        // Should time be handled in floating point or int
        let mut previous = Instant::now();
        // let mut lag = 0.0f64;
        let mut game_time = 0u64;

        let mut lag = 0.0;

        'main: loop {
            let current = Instant::now();
            let elapsed = current.duration_since(previous);
            previous = current;
            lag += elapsed.as_secs_f64();

            // Events
            // handle_events();
            // Update
            // while game_time < current {
            //     lag -= MS_PER_UPDATE;
                
            //     self.update();
            // }

            self.update();

            self.render();

            // let mut iter = self.event_receiver.try_iter();

            // Poll for events
            // while let Some(event) = iter.next() {
            //     match event {
            //         Event::Move(dx, dy) => {
            //             // self.move_entity("player", dx, dy);
            //             // self.engine.execute_action();
            //         },
            //         Event::Quit => {
            //             break 'main;
            //         }
            //     }
            // }

            previous = current;
        }

        Ok(())
    }

    fn update(&mut self) {
        // let mut engine = self.engine.borrow_mut();
        // let entities = self.engine.get_entities();
        // let num_entities = entities.len();

        // for (index, entity) in entities.iter_mut() {
        //     entity.update();
        // }

        // for (index, entity) in engine.get_entities_mut() {
        //     if let Some(action) = entity.take_turn() {
        //         action.execute(entity.borrow_mut());
        //     }

        //     // POST /entities/id/take-turn
        //     // POST /entities/id/action
        //     // POST /actions?target=""caster=""
        // }
    }

    fn render(&self) {
        // let engine = self.engine.borrow();
        let map = self.engine.get_world().get_current_map().expect("No map loaded");

        self.render_map(map);

        let entities = self.engine.get_world().get_entities();

        // Render entitys
        self.render_entities(entities);

        self.renderer.refresh();

        self.clear_entities(entities);
    }

    fn render_map(&self, map: &Map) {
        let viewport_x = 0;
        let viewport_y = 0;

        // Render the map
        // For each cell in the map
        let map_dimensions = map.get_dimensions();

        // debug!("{:?}", map.get_cells().iter().map(|cell| cell.glyph).collect::<String>());

        for y in 0..map_dimensions.height {
            for x in 0..map_dimensions.width {
                let cell = map.get_cell_ref(x, y);
                if (cell.glyph == '#') {
                    // let attr = nc::COLOR_PAIR(colors::ColorPair::WhiteBlack as i16);
                    // let attr = nc::COLOR_PAIR(1);
                    // nc::attron(attr);
                    self.renderer.mvaddch_color(viewport_x + x, viewport_y + y, cell.glyph, ColorPair::WhiteBlack);
                    // nc::attroff(attr);
                } else {
                    self.renderer.mvaddch(viewport_x + x, viewport_y + y, cell.glyph)
                }
            }
        }
    }

    fn render_entities(&self, entities: &Entities) {
        // Filter entities that have a render component
        for (id, entity) in entities {
            // info!("Rendering entity {}, {:?}", id, entity);
            self.render_entity(entity.borrow());
        }
    }

    fn render_entity(&self, entity: &dyn Entity) {
        self.renderer.mvaddch_color(
            entity.get_x(), 
            entity.get_y(), 
            entity.get_glyph(),
            entity.get_color());
    }

    fn clear_entities(&self, entities: &Entities) {
        for entity in entities.values() {
            self.clear_entity(entity.borrow());
        }
    }

    fn clear_entity(&self, entity: &dyn Entity) {
        self.renderer.mvaddch(entity.get_x(), entity.get_y(), ' ');
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