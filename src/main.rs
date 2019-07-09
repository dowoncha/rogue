#[macro_use]
extern crate log;
extern crate ncurses;

use ncurses as nc;

use std::env;
use std::panic;
use std::fs::File;
use std::io::prelude::*;

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

fn render_rect(rect: &Rect) {
    
}

fn get_input() -> i32 {
    let input = nc::getch();

    input
}

fn drop_ncurses() {
    nc::endwin();
}

struct Player {
    name: String,
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: -1,
            y: -1
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

    fn render(&self) {
        nc::mvaddch(self.y, self.x, '@' as u64);
    }
}

#[cfg(tests)]
mod player_tests {
    #[test]
    fn test_controls() {
        // 'w'
        let (dx, dy) = handle_input(119);
        assert!(dx == 0 && dy == -1);

        //' d'
        let (dx, dy) = handle_input(100);
        assert!(dx == 1 && dy == 0);

        // assert!(player.x == 101 && player.y == 99);
        // 's'
        let (dx, dy) = handle_input(115);
        assert!(dx == 0 && dy == 1);

        // 'a'
        let (dx, dy) = handle_input(97);
        assert!(dx == -1 && dy == 0);
    }

    #[test]
    fn test_change_players_name() {
        let mut player = Player::new("gromash");

        assert_eq!(player.name(), "gromash");

        player.set_name("tinker");
        
        assert_eq!(player.name(), "tinker");
    }
}

fn handle_input(input: i32) -> (i32, i32) {
    match input {
        119 => {
            //'w'
            (0, -1)
        }
        100 => {
            (1, 0)
        }
        115 => {
            (0, 1)
        }
        97 => {
            (-1, 0)
        },
        _ => { (0, 0)}
    }
}

struct Monster {
    name: String,
    x: i32,
    y: i32
}

impl Monster {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: -1,
            y: -1
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

    pub fn render(&self) {
        nc::mvaddch(self.y, self.x, 'G' as u64);
    }
}


#[test]
fn test_create_monster() {
    let monster = Monster::new("jacob");

    assert_eq!(monster.name(), "jacob");
    assert_eq!(monster.position(), (-1, -1));
}

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32
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

    fn render(&self) {
        let buffer = self.get_buffer();

        nc::mvaddstr(self.y, self.x, &buffer);
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

trait Map {
    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn render(&self);

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

struct Game {
    map: Box<dyn Map>,
    player: Player,
    monster: Monster
}

impl Game {
    pub fn new() -> Self {
        // TODO: randomly generate

        Self {
            map: Box::new(Rect { x: 0, y: 0, width: 10, height: 10 }),
            player: Player::new(""),
            monster: Monster::new("jacob")
        }
    }

    pub fn get_mut_player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub fn spawn_monster(&mut self, mut monster: Monster, x: i32, y: i32) {
        monster.set_x(x);
        monster.set_y(y);

        self.monster = monster
    }

    pub fn get_monster(&self, monster_name: &str) -> &Monster {
        &self.monster
    }
    
    pub fn serialize(&self) -> Result<String, Box<std::error::Error>> {
        use std::fmt::Write;

        let mut buffer = String::new();

        writeln!(buffer, "gromash")?;

        Ok(buffer)
    }

    pub fn save(&self) -> std::io::Result<SaveResult> {
        use std::fmt;


        let player_name = "test";
        let filename = format!("{}-datetime.save", player_name);
        let mut savefile = File::create(&filename)?;

        let buffer = self.serialize().unwrap();

        write!(savefile, "{}", buffer);

        Ok(SaveResult {
            filename: filename.to_string()
        })
    }

    pub fn render(&self) {
        self.map.render();

        self.player.render();

        self.monster.render();
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
        let monster = Monster::new("jacob");

        let mut game = Game::new();

        game.spawn_monster(monster, 5, 5);

        let monster = game.get_monster("jacob");

        assert_eq!(monster.position(), (5, 5));
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

        let mut game_save_lines = game_save_buffer.lines();

        // First line is player name
        assert_eq!(game_save_lines.next().unwrap(), "gromash");
    }

    #[test]
    fn test_save_game() {
        let game = Game::new();

        let save_result = game.save().unwrap();

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

// #[test]
// fn test_load_map_from_file() {
//     let test_mapfile = "test.map"

//     let map = load_map();

//     assert!(map.is_ok());

//     let map = map.unwrap();

//     assert!(validate_map(map));
// }

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

    game.spawn_monster(goblin, 5, 5);


    loop {
        game.render();
        
        let input = get_input();

        let (dx, dy) = handle_input(input);
        
        let (x1, y1) = {
            let player = game.get_player();

            (player.x + dx, player.y + dy)
        };

        let map = game.get_map();

        if map.is_walkable(x1, y1) {
            let player = game.get_mut_player();
            player.walk(dx, dy)
        }
    }

    game.save();

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
