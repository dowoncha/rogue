// http://www.roguebasin.com/index.php?title=C%2B%2B_Example_of_Dungeon-Building_Algorithm
use std::error;
use std::fmt;
use ::rand::{Rng};


#[derive(Debug)]
enum DungeonGenerationError {
    InvalidRectsize,
    TileOccupied
}

impl fmt::Display for DungeonGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DungeonGenerationError::InvalidRectsize => write!(f, "Rect size is invalid"),
            DungeonGenerationError::TileOccupied => write!(f, "Tile occupied error")
        }
    }
}

impl error::Error for DungeonGenerationError {
    fn description(&self) -> &str {
        match *self {
            DungeonGenerationError::InvalidRectsize => "Invalid Rect Size",
            DungeonGenerationError::TileOccupied => "Trying to place tile in occupied"
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Unused,
    Floor,
    Corridor,
    Wall,
    ClosedDoor,
    OpenDoor,
    UpStairs,
    DownStairs
}

enum Direction {
    North,
    South,
    West,
    East,
    // DirectionCount
}

impl Tile {
    pub fn value(&self) -> char {
        match *self {
            Tile::Unused => ' ',
            Tile::Floor => '.',
            Tile::Corridor => ',',
            Tile::Wall => '#',
            Tile::ClosedDoor => '+',
            Tile::OpenDoor => '-',
            Tile::UpStairs => '<',
            Tile::DownStairs => '>'
        }
    }
}

struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

pub struct Dungeon {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    rooms: Vec<Rect>,
    exits: Vec<Rect>
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Dungeon {
        Dungeon {
            width,
            height,
            tiles: Vec::new(),
            rooms: Vec::new(),
            exits: Vec::new()
        }
    }

    pub fn generate(&mut self, max_features: i32) -> Result<(), &'static str> {
        // Place first room in center

        let x = self.width / 2;
        let y = self.height / 2;

        match self.make_room(x, y, Direction::North, true) {
            Ok(()) => { },
            Err(e) => warn!("{}", e)
        }

        // Create features

        // Place UpStairs
        
        // Place DownStairs

        // For each char

        Ok(())
    }

    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
            }
        }
    }

    /// Get the tile at (x, y)
    fn get_tile(&self, x: usize, y: usize) -> Tile {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            Tile::Unused
        } else {
            self.tiles[x + y * self.width]
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x + y * self.width] = tile;
    }

    fn make_room(&mut self, x: usize, y: usize, direction: Direction, first_room: bool) -> Result<(), &'static str> {
        const MIN_ROOM_SIZE: usize = 3;
        const MAX_ROOM_SIZE: usize = 6;

        let mut rng = ::rand::thread_rng();

        let mut room = Rect {
            x: 0,
            y: 0,
            width: rng.gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
            height: rng.gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE)
        };

        match direction {
            Direction::North => {
                room.x = x - room.width / 2;
                room.y = y - room.height;
            },
            _ => { }
        }

        match self.place_rect(&room, Tile::Floor) {
            Ok(()) => {
                self.rooms.push(room);
            },
            Err(e) => return Err("Error")
        }

        Ok(())
    }

    fn place_rect(&mut self, rect: &Rect, tile: Tile) -> Result<(), DungeonGenerationError  > {
        if rect.x < 1 || rect.y < 1 || rect.x + rect.width > self.width - 1 || rect.y + rect.height > self.height - 1 {
            return Err(DungeonGenerationError::InvalidRectsize);
        }

        for y in rect.y..(rect.y + rect.height) {
            for x in rect.x..(rect.x + rect.width) {
                if self.get_tile(x, y) != Tile::Unused {
                    return Err(DungeonGenerationError::TileOccupied);
                }
            }
        }

        for y in (rect.y - 1)..(rect.y + rect.height + 1) {
            for x in (rect.x - 1)..(rect.x + rect.width + 1) {
                if x == rect.x - 1 || y == rect.y - 1 || x == rect.x + rect.width || y == rect.y + rect.height {
                    self.set_tile(x, y, Tile::Wall);
                } else {
                    self.set_tile(x, y, tile);
                }
            }
        }

        Ok(())
    }
}