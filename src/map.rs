use std::fs::File;
use std::io::prelude::*;
use engine::{GameObject, Prop, Item};
use entity::Entity;
use types::{Rect, Dimension};

#[derive(Debug)]
pub struct Cell {
    pub glyph: char,
    pub blocked: bool,
    pub block_sight: bool
    // prop: Option<Prop>,
    // item: Option<Item>,
    // entity: Option<Entity>
}

impl Cell {
    pub fn new(glyph: char, blocked: bool, block_sight: bool) -> Self {
        Self {
            blocked: blocked,
            block_sight: block_sight,
            glyph: glyph,
            // prop: None,
            // item: None,
            // entity: None
        }
    }
}

// A map is a 2d grid of tiles
pub struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = Map::init_cells(width, height);

        Self {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn init_cells(width: usize, height: usize) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(
                    '#',
                    true,
                    true
                ));
            }
        }

        cells
    }

    pub fn open(filename: &str) -> std::io::Result<Self> {
        debug!("Opening map {}", filename);

        let mut file = File::open(filename)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        let (width, height) = Map::get_buffer_dimensions(&buffer);

        let cells = Map::create_cells_from_buffer(&buffer, width, height);

        Ok(Map {
            cells: cells,
            width: width,
            height: height
        })
    }

    pub fn cell_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        let index = self.cell_index(x, y);
        debug!("x, y, index, #cells, {}, {}, {}, {}", x, y, index, self.cells.len());
        self.cells.get(index).unwrap().blocked 
        // false
    }

    fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn get_dimensions(&self) -> Dimension {
        Dimension { width: self.width as i32, height: self.height as i32 }
    }

    // Returns (width, height) of string grid buffer
    fn get_buffer_dimensions(buffer: &str) -> (usize, usize) {
        // Split each line by new line
        let lines = buffer.lines();

        // Count number of lines to get max_height
        let height = lines.count();


        // Potential optimization 
        // The first lines iterator is consumed by count
        // so have to create another one
        let lines = buffer.lines();

        let width = lines.max().unwrap().len();

        debug!("Load map buffer with dimensions (w, h): ({}, {})", width, height);

        (width, height)
    }

    fn create_cells_from_buffer(buffer: &str, width: usize, height: usize) -> Vec<Cell> {
        let lines = buffer.lines();

        let mut cells = Vec::with_capacity(width * height);

        for line in lines {
            let mut chars = line.chars();

            for _ in 0..width {
                let glyph = chars.next().unwrap_or(' ');

                match glyph {
                    '#' => {
                        cells.push(Cell::new(glyph, true, true));
                    }
                    _ => {
                        cells.push(Cell::new(glyph, false, false));
                    }
                }
            }
        }

        cells
    }

    pub fn get_cell_ref(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize * self.width + x as usize]
    }

    pub fn get_mut_cell_ref(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[y as usize * self.width + x as usize]
    }

    pub fn find_entity(&self, entity_id: &str) -> Option<&Entity> {
        None
    }
}

pub struct MapBuilder {
    width: usize,
    height: usize,
    map: Map
}

impl MapBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            map: Map::new(width, height)
        }
    }

    pub fn create_room(mut self, room: Rect) -> Self {
        let cells = &mut self.map.cells;

        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                let mut cell = &mut cells[y as usize * self.width + x as usize];
                cell.glyph = '.';
                cell.blocked = false;
                cell.block_sight = false;
            }
        }

        self
    }

    pub fn build(self) -> Map {
        self.map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use engine::EntityBuilder;

#[test]
fn test_map_get_buffer_dimensions() {
    let test = "###\n####\n#";

    let (width, height) = Map::get_buffer_dimensions(test);

    assert_eq!(width, 4);
    assert_eq!(height, 3);
}

#[test]
fn test_map_spawn_entity() {
    let mut map = Map::new();

    map.width = 2;
    map.height = 2;

    map.cells.push(Cell::new('.'));

    map.spawn_entity(0, 0, EntityBuilder::new().name("test entity").build());

    let entity = map.get_cell_ref(0, 0).get_entity_ref();

    assert!(entity.is_some());

    let entity = entity.unwrap();

    assert_eq!(entity.name, "test entity");
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

#[test]
fn test_map_create_cells_from_buffer() {
    let buffer = "####  \n#..#\n#..#\n####";

    let cells = Map::create_cells_from_buffer(buffer, 6, 4);

    assert_eq!(cells.len(), 24);
    assert_eq!(cells.iter().filter(|cell| cell.glyph == '#').count(), 12);
    assert_eq!(cells.iter().filter(|cell| cell.glyph == '.').count(), 4);
    assert_eq!(cells.iter().filter(|cell| cell.glyph == ' ').count(), 8);
}

}