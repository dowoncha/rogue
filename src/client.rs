use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};

use rand::prelude::*;
use ncurses as nc;

use game_state::GameState;
use map::{Map, MapBuilder};
use types::{BoxResult, Rect};
use action::{Action, WalkAction};
use engine::{Engine};
use renderer::{TerminalRenderer, ColorPair};
use entity::{Entity, Entities, Breed, Hero, Monster};
use world::World;

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

pub fn create_map(
    max_rooms: i32, 
    room_min_size: usize, 
    room_max_size: usize, 
    map_width: usize, 
    map_height: usize
) -> Map {
    let mut rng = rand::thread_rng();

    let mut map_builder = MapBuilder::new(map_width, map_height);

    let mut rooms = Vec::new();

    for _ in 0..max_rooms {
        let w = rng.gen_range(room_min_size, room_max_size) as i32;
        let h = rng.gen_range(room_min_size, room_max_size) as i32;

        let x = rng.gen_range(0, map_width as i32 - w - 1);
        let y = rng.gen_range(0, map_height as i32 - h - 1 );

        let new_room = Rect::new(x, y, w, h);

        let mut intersected = false;

        for other_room in &rooms {
            if new_room.intersect(other_room) {
                intersected = true;
                break;
            }
        }

        if !intersected {
            map_builder = map_builder.create_room(&new_room);

            let (new_room_center_x, new_room_center_y ) = new_room.center();

            if !rooms.is_empty() {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // flip a coin
                if rng.gen::<f32>() > 0.5 {
                    map_builder = map_builder
                        .create_h_tunnel(prev_x, new_room_center_x, prev_y)
                        .create_v_tunnel(prev_y, new_room_center_y, new_room_center_x);
                } else {
                    map_builder = map_builder
                        .create_v_tunnel(prev_y, new_room_center_y, prev_x)
                        .create_h_tunnel(prev_x, new_room_center_x, new_room_center_y);
                }
            }

            rooms.push(new_room);
        }
    }

    let mut map = map_builder.build();

    map.set_rooms(rooms);

    map
}

fn create_player(map: &Map) -> Hero {
    let mut player = Hero::new("Rand Al'Thor");
    let first_room_center = map.get_rooms()[0].center();
    let player_x = first_room_center.0;
    let player_y = first_room_center.1;
    player.set_x(player_x);
    player.set_y(player_y);

    player
}

fn create_monsters(map: &Map, max_monsters_per_room: i32) -> Vec<Box<dyn Entity>> {
    let entities = map.get_rooms().iter()
        .map(|room| gen_entities(&room, max_monsters_per_room))
        .flatten()
        .collect();

    entities
}

pub enum Event {
    Move(i32, i32),
    Quit
}

pub struct GameClient {
    renderer: TerminalRenderer,
    world: World,
    event_sender: std::sync::mpsc::Sender<Event>,
    event_receiver: std::sync::mpsc::Receiver<Event>
}

impl GameClient {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            renderer: TerminalRenderer::new(),
            event_sender: sender,
            event_receiver: receiver,
            world: World::new()
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

        let max_rooms = 20;
        let room_min_size = 5;
        let room_max_size = 20;

        let map = create_map(
            max_rooms,
            room_min_size,
            room_max_size,
            screen_width, 
            screen_height
        );

        let hero = create_player(&map);
        self.world.register_entity("player", Box::new(hero));

        let max_monsters_per_room = 3;

        let monsters = create_monsters(&map, max_monsters_per_room);

        for (index, monster) in monsters.into_iter().enumerate() {
            self.world.register_entity(&format!("monster-{}", index), monster);
        }

        self.world.set_map(map);

        Ok(())
    }

    pub fn run(&mut self) -> BoxResult<()> {
        let mut game_state = GameState::PlayerTurn;
        let mut previous_game_state = game_state;

        let mut skip_user_input = true;

        'main: loop {
            let user_input;

            self.update();

            self.render();

            // Get user input
            if !skip_user_input {
                user_input = get_user_input();
            }

            // action = player.input_handler.handle_keys(user_input, game_state)

            // Handle player actions

            // Player move action

            // Player pickup

            // Player inventory use / drop

            // Process the results stack

            // Post player turn check
        }

        Ok(())
    }

    fn update(&mut self) {
        // let mut engine = self.engine.borrow_mut();
        // let entities = self.engine.get_entities();
        // let num_entities = entities.len();

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
        // let map = self.engine.get_world().get_current_map().expect("No map loaded");
        let map = self.world.get_current_map().unwrap();

        self.render_map(map);

        let entities = self.world.get_entities();

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
        for (index, entity) in entities.iter() {
            self.clear_entity(entity.borrow());
        }
    }

    fn clear_entity(&self, entity: &dyn Entity) {
        self.renderer.mvaddch(entity.get_x(), entity.get_y(), ' ');
    }
}

fn get_user_input() -> Option<i32> {
    let user_input = nc::getch();

    Some(user_input)
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