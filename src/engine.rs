use std::fs::File;
use std::cell::RefCell;
use std::io::prelude::*;
use std::borrow::BorrowMut;

use std::time::{Duration, Instant};
use std::collections::HashMap;

use game_state::{GameStateManager, MainMenu};

use renderer::Renderer;
use character::Player;

use ncurses as nc;

// Fixed timestep of 1 / ( 60 fps) = 16 ms
// const MS_PER_UPDATE: Duration = Duration::from_millis(16);

trait GameObject {

}

struct Cell {
    glyph: char,
    prop: Option<Prop>,
    item: Option<Item>,
    entity: Option<Entity>
}

impl Cell {
    pub fn new(glyph: char) -> Self {
        Self {
            glyph: glyph,
            prop: None,
            item: None,
            entity: None
        }
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn get_entity_ref(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }
}

impl GameObject for Cell {

}

struct Prop {

}

impl GameObject for Prop {

}

struct Entity {
    inventory: Inventory
}

impl GameObject for Entity {

}

struct Inventory {
    items: HashMap<Item, i32>
}

struct Item {

}

impl GameObject for Item {

}

// A map is a 2d grid of tiles
struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    // Returns (width, height) of string grid buffer
    fn get_buffer_dimensions(buffer: &str) -> (usize, usize) {
        // Split each line by new line
        let lines = buffer.lines();

        // Count number of lines to get max_height
        let height = lines.count();

        debug!("Map height: {}", 
            height);

        // Potential optimization 
        // The first lines iterator is consumed by count
        // so have to create another one
        let lines = buffer.lines();

        let width = lines.max().unwrap().len();

        (width, height)
    }

    pub fn open(filename: &str) -> std::io::Result<Self> {
        debug!("Opening map {}", filename);

        let mut file = File::open(filename)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer);

        let (width, height) = Map::get_buffer_dimensions(&buffer);

        let lines = buffer.lines();

        let mut cells = Vec::new();

        for line in lines {
            let mut chars = line.chars();

            for _ in 0..width {
                let glyph = chars.next();

                match glyph {
                    Some(glyph) => cells.push(Cell::new(glyph)),
                    None => cells.push(Cell::new(' '))
                }
            }
        }

        Ok(Map {
            cells: cells,
            width: width,
            height: height
        })
    }

    pub fn get_cell_ref(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize * self.height + x as usize]
    }

    pub fn get_mut_cell_ref(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[y as usize * self.height + x as usize]
    }

    pub fn spawn_entity(&mut self, x: i32, y: i32, entity: Entity) {
        let cell = self.get_mut_cell_ref(x, y);
        cell.entity = Some(entity);
    }
}

#[test]
fn test_map_get_buffer_dimensions() {
    let test = "###\n####\n#";

    let (width, height) = Map::get_buffer_dimensions(test);

    assert_eq!(width, 4);
    assert_eq!(height, 3);
}

#[test]
fn test_map_open() {
    let test_filename = "assets/test.map";

    let map_result = Map::open(test_filename);

    assert!(map_result.is_ok());

    let map = map_result.unwrap();

    assert!(map.cells.len() > 0);
    assert!(map.width > 0);
    assert!(map.height > 0);
}

type BoxResult<T> = Result<T, Box<std::error::Error>>;

pub struct GameClient {
    engine: RefCell<Engine>
}

impl GameClient {
    pub fn new() -> Self {
        Self {
            engine: RefCell::new(Engine::new())
        }
    }

    pub fn init(&self) -> BoxResult<()> {
        self.engine.borrow().init();

        Ok(())
    }

    pub fn run(&self) -> BoxResult<()>  {
        self.engine.borrow_mut().run();

        Ok(())
    }

    pub fn load_map(&self, filename: &str) -> BoxResult<()> {
        self.engine.borrow_mut().load_map(filename)
    }
}

/// Main engine
struct Engine {
    renderer: Renderer,
    player: Player,
    current_map: Option<RefCell<Map>>,
    state_manager: GameStateManager,
    event_sender: std::sync::mpsc::Sender<String>,
    event_receiver: std::sync::mpsc::Receiver<String>
}

impl Engine {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let renderer = Renderer::new();
        let player = Player::new();

        Self {
            renderer: renderer,
            player: player,
            current_map: None,
            state_manager: GameStateManager::new(),
            event_sender: sender,
            event_receiver: receiver
        }
    }

    pub fn init(&self) {
        // Renderer::init();

        self.renderer.init();

        // First state is main menu
        // self.state_manager.change_state(Box::new(MainMenu::new()))
    }

    pub fn run(&mut self) {
        let mut previous = Instant::now();
        // let mut lag = 0.0f64;
        let mut game_time = 0u64;

        self.player.set_x(32);
        self.player.set_y(22);

        loop {
            //let current = Instant::now();
            // let elapsed = current.duration_since(current);
            // previous = current;
            // lag += elapsed;

            // Events
            // handle_events();
            // Update
            /*
            while game_time < current {
                lag -= MS_PER_UPDATE;
                
                self.update();
            }
            */
            let input = self.renderer.getch();

            let event = format!("key {}", input);

            self.event_sender.send(event)
                .expect("Failed to send event");

            self.update();
                        // Render
            self.render();

            // self.renderer.mvprintw(1, 1, &format!("{}", input));
        }
    }

    fn handle_input(&mut self, input: i32) {
        match input {
            119 => {
                // 'w'
                let player_y = self.player.y;
                self.player.set_y(player_y - 1);
            },
            115 => {
                // 'd'
                let player_y = self.player.y;

                self.player.set_y(player_y + 1);
            },
            100 => {
                // 'd'
                let player_x = self.player.x;
                self.player.set_x(player_x + 1);
            },
            97 => {
                // 'a'
                let player_x = self.player.x;

                self.player.set_x(player_x - 1);
            },
            113 => {
                // 'q'
                return;
            },
            _ => {}
        }
    }

    fn update(&mut self) -> Result<(), Box<std::error::Error>> {
        let event = self.event_receiver.recv().unwrap();

        info!("Event: {}", event);
        // self.renderer.mvprintw(1, 1, &format!("Event: {}", event));

        let mut args = event.split_whitespace();

        let command = args.next().unwrap_or_else(|| {
          error!("No command found in event");
          ""
        });

        debug!("Command: {}", command);

        match command {
            "key" => {
                let arg1 = args.next();

                match arg1 {
                    Some(keycode) => {
                        let keycode = keycode.parse::<i32>().unwrap();

                        self.handle_input(keycode);
                    },
                    None => {
                        warn!("No keycode found for key command");
                    }
                }
            }
            _ => {
                debug!("Unrecognized command {}", command);
                // self.renderer.mvprintw(1, 1, &format!("Unrecognized command: {}", command));
            }
        };

        // self.state_manager.update();

        Ok(())
    }

    fn render(&self) {
        self.renderer.clear();

        self.renderer.mvprintw(30, 20, "#################");

        for y in 21..30 {
            self.renderer.mvprintw(30, y, "#               #");
        }

        self.renderer.mvprintw(30, 30, "#################");

        self.player.render(&self.renderer);

        self.renderer.refresh();
    }

    pub fn load_map(&mut self, filename: &str) -> BoxResult<()> {
        let map = Map::open(filename)?;

        self.current_map = Some(RefCell::new(map));

        Ok(())
    }

    fn spawn_entity(&self, x: i32, y: i32, entity: Entity) -> BoxResult<()> {
        match self.current_map {
            Some(ref map) => {
                map.borrow_mut().spawn_entity(x, y, entity);

                return Ok(())
            }
            None => {
                return Err(Box::new(EngineError::NoLoadedMap));
            }
        }
    }

    // fn remove_entity(&self, x: i32, y: i32, entity: Entity) {
    //     let cell = self.map.get_cell(x, y);
    //     let entity = cell.get_entity();

    //     if let Some(entity) = entity {
    //         cell.entity = None
    //     }
    // }
}

#[derive(Debug)]
enum EngineError {
    NoLoadedMap,
    EntityNotFound
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EngineError::NoLoadedMap => write!(f, "No map loaded"),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl std::error::Error for EngineError {

}