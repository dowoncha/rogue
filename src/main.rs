#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
#[macro_use]
extern crate lazy_static;


extern crate rogue;
use rogue::{file_logger, Component, System, InputSystem, EntityManager, ComponentType};
use rogue::components::{Position, Input};

use ncurses as nc;

use std::any::{Any, TypeId};
use std::collections::{HashMap, LinkedList, VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

/**
 * Render code
 */

fn init_ncurses() {
    // Start ncurses
    nc::initscr();

    if !nc::has_colors() {
        nc::endwin();
        error!("Terminal does not support color");
        // return Err(Box::new("Terminal does not support color".to_string()));
    }

    // Allow colors
    nc::start_color();

    // colors::init();

    // Line buffering disabled
    // Signals are not interpreted and are instead passed directly to program
    // TODO: change to raw after implementing signals
    // nc::raw();
    nc::cbreak();

    // Disable echoing of chracaters
    nc::noecho();

    // Enableds reading of function keys
    nc::keypad(nc::stdscr(), true);

    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn drop_ncurses() {
    nc::endwin();
}

pub struct Render {
    pub glyph: char,
    // fg
    // bg
}

impl Component for Render {
    fn get_component_type() -> ComponentType {
        ComponentType::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct RenderSystem;

impl RenderSystem {
    fn new() -> Self {
        Self {}
    }
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        drop_ncurses();
    }
}

impl System for RenderSystem {
    fn mount(&mut self) {
        init_ncurses();
    }

    fn process(&mut self, entity_manager: &mut EntityManager) {
        let render_entities = entity_manager.get_entities_with_components(Render::get_component_type());

        for render_entity in render_entities {
            let component = entity_manager.get_component(render_entity, Render::get_component_type()).unwrap();
            let render = component.as_any().downcast_ref::<Render>().unwrap();
            
            let component = entity_manager.get_component(render_entity, Position::get_component_type()).unwrap();
            let position = component.as_any().downcast_ref::<Position>().unwrap();

            nc::mvaddch(position.y, position.x, render.glyph as u64);
        }

        // self.map.render();

//         self.player.render();

//         for monster in self.monsters.values() {
//             monster.render();
//         }
    }
}

/** Entity code */
trait Health {
    fn max_health(&self) -> i32;
    fn health(&self) -> i32;
    fn set_health(&mut self, health: i32);
    fn is_dead(&self) -> bool {
        self.health() <= 0
    }
}

struct Player {
    name: String,
    pub x: i32,
    pub y: i32,
    health: i32,
    max_health: i32,
    strength: i32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: -1,
            y: -1,
            health: 15,
            max_health: 15,
            strength: 10,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn walk(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn calc_damage(&self, monster: &Monster) -> i32 {
        // DND sysetem

        // Roll weapon dice
        3
    }

    fn attack(&self, monster: &mut Monster) {
        let damage = self.calc_damage(monster);
        let new_health = monster.health() - damage;
        monster.set_health(new_health);
    }
}

// impl Render for Player {
//     fn render(&self) {
//         nc::mvaddch(self.y, self.x, '@' as u64);
//     }
// }

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn test_change_players_name() {
        let mut player = Player::new("gromash");

        assert_eq!(player.name(), "gromash");

        player.set_name("tinker");

        assert_eq!(player.name(), "tinker");
    }

    #[test]
    fn test_player_attack() {
        let player = Player::new("attacker");

        let mut monster = Monster::new("defender");

        assert_eq!(monster.max_health(), 10);
        assert_eq!(monster.health(), 10);

        player.attack(&mut monster);

        assert_eq!(monster.max_health(), 10);
        assert_eq!(monster.health(), 7);

        player.attack(&mut monster);

        assert_eq!(monster.health(), 4);

        player.attack(&mut monster);

        assert_eq!(monster.health(), 1);

        player.attack(&mut monster);

        assert!(monster.is_dead());
    }
}

#[derive(Debug)]
struct Monster {
    name: String,
    x: i32,
    y: i32,
    max_health: i32,
    health: i32,
}

impl Monster {
    pub fn new(name: &str) -> Self {
        let max_health = 10;

        Self {
            name: name.to_string(),
            x: -1,
            y: -1,
            max_health: max_health,
            health: max_health,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl Health for Monster {
    fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn max_health(&self) -> i32 {
        self.max_health
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// impl Render for Monster {
//     fn render(&self) {
//         nc::mvaddch(self.y, self.x, 'G' as u64);
//     }
// }

impl Timed for Monster {
    fn rate(&self) -> i32 {
        let speed = 10;
        speed
    }

    fn cost(&mut self) -> i32 {
        // Do something

        // how much did the action cost?
        0
    }
}

#[test]
fn test_create_monster() {
    let monster = Monster::new("jacob");

    assert_eq!(monster.name(), "jacob");
    assert_eq!(monster.position(), (-1, -1));
}

#[test]
fn test_monster_equality() {
    let monster1 = Monster::new("jacob");

    let monster2 = Monster::new("jacob");

    assert_eq!(monster1, monster2);
}

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

// impl Render for Rect {
//     fn render(&self) {
//         let buffer = self.get_buffer();

//         nc::mvaddstr(self.y, self.x, &buffer);
//     }
// }

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

            buffer.push('\n');
        }

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
        std::fs::write(filename, "MAP\nENDMAP\n");

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
    player: Player,
    monsters: HashMap<String, Monster>,
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
            }),
            player: Player::new(""),
            monsters: HashMap::new(),
            // entities: HashMap::new(),
        }
    }

    pub fn get_mut_player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn move_player(&mut self, x: i32, y: i32) -> Result<(), MoveError> {
        if !self.map.is_walkable(x, y) {
            return Err(MoveError::Blocked("tile".to_string()));
        }

        let mut blocker = None;

        for monster in self.get_monsters() {
            if monster.position() == (x, y) {
                blocker = Some(monster.name().to_string());
            }
        }

        // Player should attack blocking monster
        if let Some(monster_name) = blocker {
            let mut monster = self.monsters.get_mut(&monster_name).unwrap();

            self.player.attack(&mut monster);

            return Err(MoveError::Blocked(monster_name.to_string()));
        }

        self.player.set_position(x, y);

        Ok(())
    }

    // pub fn register_entity(&mut self, id: &str, entity: impl Entity + 'static) {
    //     self.entities.insert(id.to_string(), Box::new(entity));
    // }

    // pub fn get_entities(&self) -> impl Iterator<Item = &Box<dyn Entity>> {
    //     self.entities.values()
    // }

    // pub fn get_entities_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Entity>> {
    //     self.entities.values_mut()
    // }

    pub fn spawn_monster(&mut self, mut monster: Monster, x: i32, y: i32) {
        // TODO:
        // Cannot spawn if entity exists on space

        monster.set_x(x);
        monster.set_y(y);

        self.monsters.insert(monster.name().to_string(), monster);
    }

    pub fn get_monster(&self, monster_name: &str) -> Option<&Monster> {
        self.monsters
            .values()
            .find(|monster| monster.name() == monster_name)
    }

    pub fn get_monsters_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Monster> {
        self.monsters.values_mut()
    }

    pub fn get_monsters<'a>(&'a self) -> impl Iterator<Item = &'a Monster> {
        self.monsters.values()
    }

    pub fn serialize(&self) -> Result<String, Box<std::error::Error>> {
        use std::fmt::Write;

        let mut buffer = String::new();

        // write game header
        writeln!(buffer, "Rogue")?;

        let game_version = 0.1;

        // write game version
        writeln!(buffer, "v{}", game_version)?;

        // Write player info
        writeln!(buffer, "{}", self.player.name())?;

        // write save_date

        // write map
        writeln!(buffer, "MAP\n{}\nENDMAP", &self.get_map().get_buffer())?;

        // write entities
        for (id, monster) in self.monsters.iter() {
            writeln!(buffer, "ENTITY {}", id)?;
            writeln!(buffer, "name\t{}", monster.name())?;
            writeln!(
                buffer,
                "hp/max\t{}/{}",
                monster.health(),
                monster.max_health()
            )?;
            writeln!(buffer, "ENDENTITY")?;
        }

        Ok(buffer)
    }

    pub fn save(&self, filename: Option<&str>) -> std::io::Result<SaveResult> {
        use std::fmt::Write;

        let player_name = self.player.name();
        let filename = format!("{}-datetime.save", player_name);
        let mut savefile = File::create(&filename)?;

        let buffer = self.serialize().unwrap();

        write!(savefile, "{}", buffer)?;

        Ok(SaveResult {
            filename: filename.to_string(),
        })
    }

    pub fn cleanup(&mut self) {
        self.monsters.retain(|k, monster| !monster.is_dead());
    }

    pub fn get_map(&self) -> &dyn Map {
        &*self.map
    }

    pub fn set_map(&mut self, map: impl Map + 'static) {
        self.map = Box::new(map);
    }

    pub fn spawn_player(&mut self) {
        self.player.set_x(5);
        self.player.set_y(5);
    }
}

// impl Render for Game {
//     fn render(&self) {
//         
//     }
// }

/**
 * Chronos is the time keeper
 * He has a reference to all entities
 * And allocates time to entities for actions
 */
struct Chronos {
    time_travelers: VecDeque<(i32, String)>,
}

impl Chronos {
    pub fn new() -> Self {
        Self {
            time_travelers: VecDeque::new(),
        }
    }

    pub fn travelers_len(&self) -> usize {
        self.time_travelers.len()
    }

    pub fn register(&mut self, entity_id: &str) {
        let current_time = -100;
        self.time_travelers
            .push_back((current_time, entity_id.to_string()));
    }

    pub fn release(&mut self, entity_id: &str) {
        if let Some(index) = self
            .time_travelers
            .iter()
            .position(|(energy, traveler)| traveler == entity_id)
        {
            self.time_travelers.remove(index);
        }
    }

    pub fn tick<'m>(&mut self, entities: impl Iterator<Item = &'m mut Box<dyn Timed>>) {}
}

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

        game.get_mut_player().set_name("gromash");

        assert_eq!(game.get_player().name(), "gromash");

        game.get_mut_player().set_name("tinker");

        assert_eq!(game.get_player().name(), "tinker");
    }

    #[test]
    fn test_spawn_player() {
        let room = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let mut game = Game::new();

        game.set_map(room);

        game.spawn_player();

        let player = game.get_player();
        let room = game.get_map();

        assert!(
            player.x > 0 && player.x < room.width() && player.y > 0 && player.y < room.height()
        );
    }

    #[test]
    fn test_spawn_monster() {
        let monster_name = "jacob";
        let monster = Monster::new(monster_name);

        let mut game = Game::new();

        game.spawn_monster(monster, 5, 5);

        let monster = game.get_monster(monster_name);

        assert!(monster.is_some());

        assert_eq!(monster.unwrap().position(), (5, 5));
    }

    #[test]
    fn test_player_move() {
        let mut game = Game::new();

        let map = Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        game.set_map(map);

        let walk_result = game.move_player(5, 5);

        assert!(walk_result.is_ok());
    }

    #[test]
    fn test_player_move_into_monster_tile() {
        let mut game = Game::new();
        let monster_name = "jacob";

        let monster = Monster::new(monster_name);

        game.spawn_monster(monster, 6, 6);

        let walk_result = game.move_player(6, 6);

        assert_eq!(
            walk_result,
            Err(MoveError::Blocked(monster_name.to_string()))
        );
    }

    #[test]
    fn test_reaper() {
        let mut game = Game::new();

        game.cleanup();

        assert!(game.get_monsters().all(|monster| !monster.is_dead()));
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

        game.get_mut_player().set_name("gromash");

        let game_save_buffer = game.serialize().unwrap();

        assert!(validate_save(&game_save_buffer));
    }

    #[test]
    fn test_save_game() {
        let mut game = Game::new();

        game.get_mut_player().set_name("gromash");

        let save_result = game.save(None).unwrap();

        let savefilename = save_result.filename;

        let savefile_buffer = &std::fs::read(&savefilename).expect("Save file not found");

        let savefile_str = String::from_utf8_lossy(&savefile_buffer);

        assert_eq!(savefile_str, game.serialize().unwrap());

        std::fs::remove_file(savefilename).unwrap();
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

trait Timed {
    fn rate(&self) -> i32 {
        0
    }
    fn cost(&mut self) -> i32 {
        0
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

impl Timed for GameTime {
    fn rate(&self) -> i32 {
        100
    }

    fn cost(&mut self) -> i32 {
        *self = self.tick();

        1000
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

    em.add_component(player, Input);
    em.add_component(player, Render { glyph: '@' });
    em.add_component(player, Position{ x: 0, y: 0});
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

    render_system.mount();
    input_system.mount();

    create_player(&mut entity_manager);

    let mut chronos = Chronos::new();

    let rect = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 20,
    };

    game.set_map(rect);

    game.get_mut_player().set_name("gromash");
    game.spawn_player();

    let goblin = Monster::new("goblin");

    game.spawn_monster(goblin, 7, 5);

    let game_time = GameTime::new();

    // game.register_entity("gametime", game_time);

    'main: loop {
        input_system.process(&mut entity_manager);

        render_system.process(&mut entity_manager);

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

        game.cleanup();
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
