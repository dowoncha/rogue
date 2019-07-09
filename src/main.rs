#[macro_use]
extern crate log;
extern crate ncurses;

use ncurses as nc;

// extern crate rogue;

use std::env;

// use rogue::{GameClient};
// use rogue::file_logger;

use std::panic;

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

fn render_player(player: &Player) {
    nc::mvaddch(player.y, player.x, '@' as u64);
}

fn render_rect(rect: &Rect) {
    for y in rect.y..(rect.y + rect.height) {
        for x in rect.x..(rect.x + rect.width) {
            if y == rect.y || y == rect.y + rect.height - 1 {
                nc::mvaddch(y, x, '-' as u64);
            } else if (x == rect.x || x == rect.x + rect.width - 1) {
                nc::mvaddch(y, x, '|' as u64);
            } else {
                nc::mvaddch(y, x, '.' as u64);
            }
        }
    }
}

fn get_input() -> i32 {
    let input = nc::getch();

    input
}

fn drop_ncurses() {
    nc::endwin();
}

struct Player {
    x: i32,
    y: i32
}

impl Player {
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

fn move_player(player: &mut Player, dx: i32, dy: i32) {
    player.x += dx;
    player.y += dy;
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

fn spawn_player_in_room(player: &mut Player, room: &Rect) {
    player.set_x(5);
    player.set_y(5);
}

fn is_walkable(room: &Rect, x: i32, y: i32) -> bool {
    x > 0 && x < 19 && y > 0 && y < 19
}

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
fn test_spawn_player() {
    let room = Rect {
        x: 0, 
        y: 0,
        width: 10,
        height: 10
    };

    let mut player = Player {
        x: -1,
        y: -1
    };

    spawn_player_in_room(&mut player, &room);

    assert!(player.x > 0 && player.x < room.width && player.y > 0 && player.y < room.height);
}

#[test]
fn test_rect_room_collision() {
    let room = Rect { x: 0, y: 0, width: 20, height: 20 };

    for i in 0..20 {
        assert!(!is_walkable(&room, i, 0));
        assert!(!is_walkable(&room, i, 20));
        assert!(!is_walkable(&room, 0, i));
        assert!(!is_walkable(&room, 20, i));
    }

    for y in 1..19 {
        for x in 1..19 {
            assert!(is_walkable(&room, x, y));
        }
    }
}

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    let args: Vec<_> = env::args().collect();

    // file_logger::init()
    //     .expect("Failed to init file logger");

    // Initialize ncurses
    init_ncurses();

    let rect = Rect { x: 0, y: 0, width: 20, height: 20 };

    let mut player = Player { x: -1, y: -1 };

    spawn_player_in_room(&mut player, &rect);

    loop {
        render_rect(&rect);

        render_player(&player);

        let input = get_input();

        let (dx, dy) = handle_input(input);
        
        let (x1, y1) = (player.x + dx, player.y + dy);

        if is_walkable(&rect, x1, y1) {
            move_player(&mut player, dx, dy);
        }
    }

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