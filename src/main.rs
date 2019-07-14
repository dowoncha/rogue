#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;

#[macro_use]
extern crate rogue;
use rogue::{
    file_logger, 
    Component, 
    ComponentType, 
    Entity, 
    EntityManager, 
    drop_ncurses, 
    System, 
    InputSystem, 
    RenderSystem, 
    MovementSystem,
    Chronos, 
};

use rogue::event_system::{EventSystem};
use rogue::command_system::{CommandSystem};
use rogue::components::{Position, Input, Render, RenderLayer, Collidable};

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

/** Entity code */
trait Health {
    fn max_health(&self) -> i32;
    fn health(&self) -> i32;
    fn set_health(&mut self, health: i32);
    fn is_dead(&self) -> bool {
        self.health() <= 0
    }
}

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Map for Rect {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn is_walkable(&self, x: i32, y: i32) -> bool {
        x > self.x && x < self.x + self.width - 1 && y > 0 && y < self.y + self.height - 1
    }

    fn get_buffer(&self) -> String {
        let mut buffer = String::new();

        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                if y == self.y || y == self.y + self.height - 1 {
                    // nc::mvaddch(y, x, '-' as u64);
                    buffer.push('-');
                } else if x == self.x || x == self.x + self.width - 1 {
                    // nc::mvaddch(y, x, '|' as u64);
                    buffer.push('|');
                } else {
                    // nc::mvaddch(y, x, '.' as u64);
                    buffer.push('.');
                }
            }
        }

        assert_eq!(buffer.len(), (self.width() * self.height()) as usize);

        buffer
    }

    fn save(&self, filename: &str) -> std::io::Result<SaveResult> {
        use std::fmt::Write;

        let mut file = File::create(filename)?;

        write!(file, "{}", self.get_buffer());

        Ok(SaveResult {
            filename: filename.to_string(),
        })
    }
}

trait Map {
    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn is_walkable(&self, x: i32, y: i32) -> bool;

    fn save(&self, filename: &str) -> std::io::Result<SaveResult> {
        // let mut savefile = File::create(filename).unwrap();
        std::fs::write(filename, "MAP\nENDMAP\n")?;

        Ok(SaveResult {
            filename: filename.to_string(),
        })
    }

    fn get_buffer(&self) -> String;
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn test_rect_room_collision() {
        let room = Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        };

        for i in 0..20 {
            assert!(!room.is_walkable(i, 0));
            assert!(!room.is_walkable(i, 20));
            assert!(!room.is_walkable(0, i));
            assert!(!room.is_walkable(20, i));
        }

        for y in 1..19 {
            for x in 1..19 {
                assert!(room.is_walkable(x, y));
            }
        }
    }
}

struct SaveResult {
    filename: String,
}

#[derive(Debug, PartialEq)]
enum MoveError {
    Blocked(String),
}

struct Game {
    map: Box<dyn Map>,
    // entities: HashMap<String, Box<dyn Entity>>,
}

impl Game {
    pub fn new() -> Self {
        // TODO: randomly generate

        Self {
            map: Box::new(Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 10,
            })
            // entities: HashMap::new(),
        }
    }

    // pub fn move_player(&mut self, x: i32, y: i32) -> Result<(), MoveError> {
    //     if !self.map.is_walkable(x, y) {
    //         return Err(MoveError::Blocked("tile".to_string()));
    //     }

    //     let mut blocker = None;

    //     for monster in self.get_monsters() {
    //         if monster.position() == (x, y) {
    //             blocker = Some(monster.name().to_string());
    //         }
    //     }

    //     // Player should attack blocking monster
    //     if let Some(monster_name) = blocker {
    //         let mut monster = self.monsters.get_mut(&monster_name).unwrap();

    //         self.player.attack(&mut monster);

    //         return Err(MoveError::Blocked(monster_name.to_string()));
    //     }

    //     self.player.set_position(x, y);

    //     Ok(())
    // }

    // pub fn register_entity(&mut self, id: &str, entity: impl Entity + 'static) {
    //     self.entities.insert(id.to_string(), Box::new(entity));
    // }

    // pub fn get_entities(&self) -> impl Iterator<Item = &Box<dyn Entity>> {
    //     self.entities.values()
    // }

    // pub fn get_entities_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Entity>> {
    //     self.entities.values_mut()
    // }

    pub fn serialize(&self) -> Result<String, Box<std::error::Error>> {
        use std::fmt::Write;

        let mut buffer = String::new();

        // write game header
        writeln!(buffer, "Rogue")?;

        let game_version = 0.1;

        // write game version
        writeln!(buffer, "v{}", game_version)?;

        // Write player info
        // writeln!(buffer, "{}", self.player.name())?;

        // write save_date

        // write map
        writeln!(buffer, "MAP\n{}\nENDMAP", &self.get_map().get_buffer())?;

        // write entities
        // for (id, monster) in self.monsters.iter() {
        //     writeln!(buffer, "ENTITY {}", id)?;
        //     writeln!(buffer, "name\t{}", monster.name())?;
        //     writeln!(
        //         buffer,
        //         "hp/max\t{}/{}",
        //         monster.health(),
        //         monster.max_health()
        //     )?;
        //     writeln!(buffer, "ENDENTITY")?;
        // }

        Ok(buffer)
    }

    pub fn save(&self, filename: Option<&str>) -> std::io::Result<SaveResult> {
        use std::fmt::Write;

        // let player_name = self.player.name();
        // let filename = format!("{}-datetime.save", player_name);
        let filename = "game.save";
        let mut savefile = File::create(&filename)?;

        let buffer = self.serialize().unwrap();

        write!(savefile, "{}", buffer)?;

        Ok(SaveResult {
            filename: filename.to_string(),
        })
    }

    pub fn get_map(&self) -> &dyn Map {
        &*self.map
    }

    pub fn set_map(&mut self, map: impl Map + 'static) {
        self.map = Box::new(map);
    }
}

// impl Render for Game {
//     fn render(&self) {
//         
//     }
// }

fn validate_save(buffer: &str) -> bool {
    let mut lines = buffer.lines();
    if lines.next().unwrap() != "Rogue" {
        return false;
    }

    true
}

#[cfg(test)]
mod game_tests {
    use super::*;

    struct TestEntity;

    #[test]
    fn test_set_player_name() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // assert_eq!(game.get_player().name(), "gromash");

        // game.get_mut_player().set_name("tinker");

        // assert_eq!(game.get_player().name(), "tinker");
    }

    #[test]
    fn test_spawn_player() {
        // let room = Rect {
        //     x: 0,
        //     y: 0,
        //     width: 10,
        //     height: 10,
        // };

        // let mut game = Game::new();

        // game.set_map(room);

        // game.spawn_player();

        // let player = game.get_player();
        // let room = game.get_map();

        // assert!(
        //     player.x > 0 && player.x < room.width() && player.y > 0 && player.y < room.height()
        // );
    }

    #[test]
    fn test_time() {
        let mut chronos = Chronos::new();

        let entity_id = "";

        chronos.register(entity_id);

        chronos.release(entity_id);
    }

    // player movement into monster occupied tile */
    // Moving into monster's tile attacks it */
}

#[cfg(test)]
mod save_load_tests {
    use super::*;

    #[test]
    fn test_serialize_game() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // let game_save_buffer = game.serialize().unwrap();

        // assert!(validate_save(&game_save_buffer));
    }

    #[test]
    fn test_save_game() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // let save_result = game.save(None).unwrap();

        // let savefilename = save_result.filename;

        // let savefile_buffer = &std::fs::read(&savefilename).expect("Save file not found");

        // let savefile_str = String::from_utf8_lossy(&savefile_buffer);

        // assert_eq!(savefile_str, game.serialize().unwrap());

        // std::fs::remove_file(savefilename).unwrap();
    }

    /* test load game */
    /**  test deserialize game **/

    #[test]
    fn test_save_rect_map_to_file() {
        let filename = "test-save-rect-map-to-file.map";

        let map = Rect {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };

        let save_result = map.save(filename);

        assert!(save_result.is_ok());

        let loadedmap_buffer = std::fs::read(filename);

        assert!(loadedmap_buffer.is_ok());

        // assert!(loadedmap_buffer.unwrap() == map.get_buffer());

        // cleanup
        std::fs::remove_file(filename);

        assert!(std::fs::read(filename).is_err());
    }
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



fn create_player(em: &mut EntityManager) {
    let player = em.create_entity();

    em.add_component(player, Input::new());
    em.add_component(player, Render { glyph: '@', layer: RenderLayer::Player });
    em.add_component(player, Position{ x: 0, y: 0});
    em.add_component(player, Collidable);
}

fn create_monster(em: &mut EntityManager, x: i32, y: i32) {
    let monster = em.create_entity();

    em.add_component(monster, Render { glyph: 'G', layer: RenderLayer::Player});
    em.add_component(monster, Position { x: x, y: y });
}

fn create_map(entity_manager: &mut EntityManager ) {
    let map = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 20,
    };

    let buffer = map.get_buffer();
    let mut tiles = buffer.chars();

    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = entity_manager.create_entity();
            let index = (y * map.width() + x) as usize;

            let glyph = tiles.next().expect("No tile at index");

            entity_manager.add_component(tile, Render { glyph: glyph, layer: RenderLayer::Map });
            entity_manager.add_component(tile, Position{ x: x, y: y });

            if glyph == '-' || glyph == '|' {
                entity_manager.add_component(tile, Collidable);
            }
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    // Initialize ncurses
    // init_ncurses();

    let mut game = Game::new();

    let mut entity_manager = EntityManager::new();
    let mut render_system = RenderSystem::new();
    let mut input_system = InputSystem::new();
    let mut movement_system = MovementSystem::new();
    let mut event_system = EventSystem::new();
    let mut command_system = CommandSystem::new();
    let mut chronos = Chronos::new();

    render_system.mount();
    input_system.mount();
    event_system.mount(&mut entity_manager);
    command_system.mount(&mut entity_manager);

    create_map(&mut entity_manager);
    create_player(&mut entity_manager);
    create_monster(&mut entity_manager, 5, 7);

    // let game_time = GameTime::new();

    // game.register_entity("gametime", game_time);

    'main: loop {
        input_system.process(&mut entity_manager);

        event_system.process(&mut entity_manager);

        movement_system.process(&mut entity_manager);

        command_system.process(&mut entity_manager);

        render_system.process(&mut entity_manager);

        chronos.process(&mut entity_manager);

        event_system.cleanup(&mut entity_manager);

        // Update
        // Whos turn is it?

        // this needs to be moved into player's turn
        // let input = get_input();

        // if let Some(input_event) = handle_input(input) {
        //     match input_event {
        //         InputEvent::MovePlayer(dx, dy) => {
        //             let (x1, y1) = {
        //                 let player = game.get_player();

        //                 (player.x + dx, player.y + dy)
        //             };

        //             let _ = game.move_player(x1, y1);
        //         }
        //         InputEvent::Quit => break 'main
        //     }
        // }
    }

    // game.save(None)?;
    drop_ncurses();

    Ok(())

    // let mut game = GameClient::new();
    // game.init(
    //     args
    // ).expect("Failed to init game");

    // if let Err(error) = game.run() {
    //     error!("{}", error);
    // }

    // Ok(())
}
