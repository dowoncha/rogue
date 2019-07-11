#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;

use ncurses as nc;

use std::env;
use std::panic;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, LinkedList, VecDeque};

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

fn get_input() -> i32 {
    let input = nc::getch();

    input
}

fn drop_ncurses() {
    nc::endwin();
}


trait Render {
    fn render(&self);
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
    strength: i32
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: -1,
            y: -1,
            health: 15,
            max_health: 15,
            strength: 10
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

    fn set_position(&mut self, x: i32, y: i32 ) {
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

impl Render for Player {
    fn render(&self) {
        nc::mvaddch(self.y, self.x, '@' as u32);
    }
}

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn test_controls() {
        // 'w'
        let event = handle_input(119).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(0, -1));

        //' d'
        let event = handle_input(100).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(1, 0));

        // assert!(player.x == 101 && player.y == 99);
        // 's'
        let event = handle_input(115).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(0, 1));

        // 'a'
        let event = handle_input(97).unwrap();
        assert_eq!(event, InputEvent::MovePlayer(-1, 0));

        // 'q'
        let event = handle_input(113).unwrap();
        assert_eq!(event, InputEvent::Quit);
    }
    

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

#[derive(Debug, PartialEq)]
enum InputEvent {
    MovePlayer(i32, i32),
    Quit
}

fn handle_input(input: i32) -> Option<InputEvent> {
    match input {
        119 => {
            //'w'
            Some(InputEvent::MovePlayer(0, -1))
        }
        100 => {
            Some(InputEvent::MovePlayer(1, 0))
        }
        115 => {
            Some(InputEvent::MovePlayer(0, 1))
        }
        97 => {
            Some(InputEvent::MovePlayer(-1, 0))
        }
        113 => {
            Some(InputEvent::Quit)
        }
        _ => { None }
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

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
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

impl Render for Monster {
    fn render(&self) {
        nc::mvaddch(self.y, self.x, 'G' as u32);
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
    height: i32
}

impl Render for Rect {
    fn render(&self) {
        let buffer = self.get_buffer();

        nc::mvaddstr(self.y, self.x, &buffer);
    }
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
                } else if (x == self.x || x == self.x + self.width - 1) {
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

    fn save(&self, filename: &str ) -> std::io::Result<SaveResult> {
        use std::fmt::Write;

        let mut file = File::create(filename)?;

        write!(file, "{}", self.get_buffer());

        Ok(SaveResult {
            filename: filename.to_string()
        })
    }
}

trait Map: Render {
    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn is_walkable(&self, x: i32, y: i32) -> bool;

    fn save(&self, filename: &str) -> std::io::Result<SaveResult> {
        // let mut savefile = File::create(filename).unwrap();
        std::fs::write(filename, "MAP\nENDMAP\n");

        Ok(SaveResult { filename: filename.to_string() })
    }

    fn get_buffer(&self) -> String;
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn test_rect_room_collision() {
        let room = Rect { x: 0, y: 0, width: 20, height: 20 };

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
    filename: String
}

#[derive(Debug, PartialEq)]
enum MoveError {
    Blocked(String)
}

struct Game {
    map: Box<dyn Map>,
    player: Player,
    monsters: HashMap<String, Monster>
}

impl Game {
    pub fn new() -> Self {
        // TODO: randomly generate

        Self {
            map: Box::new(Rect { x: 0, y: 0, width: 10, height: 10 }),
            player: Player::new(""),
            monsters: HashMap::new() 
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
    
    pub fn spawn_monster(&mut self, mut monster: Monster, x: i32, y: i32) {
        // TODO:
        // Cannot spawn if entity exists on space

        monster.set_x(x);
        monster.set_y(y);

        self.monsters.insert(monster.name().to_string(), monster);
    }

    pub fn get_monster(&self, monster_name: &str) -> Option<&Monster> {
        self.monsters.values().find(|monster| monster.name() == monster_name)
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
            writeln!(buffer, "hp/max\t{}/{}", monster.health(), monster.max_health())?;
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
            filename: filename.to_string()
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

impl Render for Game {
    fn render(&self) {
        self.map.render();

        self.player.render();

        for monster in self.monsters.values() {
            monster.render();
        }
    }
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
            height: 10
        };

        let mut game = Game::new();

        game.set_map(room);

        game.spawn_player();

        let player = game.get_player();
        let room = game.get_map();

        assert!(player.x > 0 && player.x < room.width() && player.y > 0 && player.y < room.height());
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

        let map = Rect { x: 0, y: 0, width: 50, height: 50 };
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

        assert_eq!(walk_result,  Err(MoveError::Blocked(monster_name.to_string())));
    }

    #[test]
    fn test_reaper() {
        let mut game = Game::new();

        game.cleanup();

        assert!(game.get_monsters().all(|monster| !monster.is_dead()));
    }

    // player movement into monster occupied tile */
    // Moving into monster's tile attacks it */
}

/* Game has a running state */

/* Game has a player */

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

        let map = Rect { x: 0, y: 0, width: 3, height: 3 };

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
    fn set_action_points(&mut self, points: i32);
    fn action_points(&self) -> i32;
    fn speed(&self) -> i32;
    fn take_turn(&mut self) -> i32 {
        0
    }
}

pub struct TimeTravelers<'t>(VecDeque<&'t dyn Timed>);

impl<'t> TimeTravelers<'t> {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn register(&mut self, entity: &'t mut dyn Timed) {
        entity.set_action_points(0);
        self.0.push_back(entity);
    }

    fn tick(&mut self) {
        if let Some(entity) = self.0.front_mut() {
            // self.0.rotate_right(1);
            let new_action_points = entity.action_points() + entity.speed();

            // entity.set_action_points(new_action_points);
        }
    }
}

pub struct TestTimed;

impl Timed for TestTimed {
    fn set_action_points(&mut self, points: i32) {}
    fn action_points(&self) -> i32 {
        1
    }

    fn speed(&self) -> i32 {
        1
    }

    
}

#[test]
fn test_register_timed_entity() {
    let mut test_timed = TestTimed;

    let mut time_travelers = TimeTravelers::new();

    time_travelers.register(&mut test_timed);

    time_travelers.tick();
}

// fn test_load_map_from_file()

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    let args: Vec<_> = env::args().collect();

    // file_logger::init()
    //     .expect("Failed to init file logger");

    // Initialize ncurses
    init_ncurses();

    let mut game = Game::new();

    let rect = Rect { x: 0, y: 0, width: 20, height: 20 };

    game.set_map(rect);

    game.get_mut_player().set_name("gromash");
    game.spawn_player();

    let goblin = Monster::new("goblin");

    game.spawn_monster(goblin, 7, 5);

    let mut game_time = 0;

    'main: loop {
        game.render();
        
        let input = get_input();

        if let Some(input_event) = handle_input(input) {
            match input_event {
                InputEvent::MovePlayer(dx, dy) => {
                    let (x1, y1) = {
                        let player = game.get_player();

                        (player.x + dx, player.y + dy)
                    };

                    let _ = game.move_player(x1, y1);
                }
                InputEvent::Quit => break 'main
            }
        }

        game.cleanup();

        game_time += 1;
    }

    game.save(None)?;

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
