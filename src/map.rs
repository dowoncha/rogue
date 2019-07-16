use std::fs::File;
use std::io::prelude::*;

use rand::{thread_rng, Rng};

use types::{Rect, Dimension};

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub glyph: char,
    pub blocked: bool,
    pub block_sight: bool
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
    rooms: Vec<Rect>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            cells: vec![Cell { glyph: '#', blocked: false, block_sight: false }; width * height],
            rooms: Vec::new()
        }
    }

    pub fn fill(&mut self, cell: Cell) {
        self.cells = vec![cell; self.width * self.height];
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /**
     * Fill room with walls
     */
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

    pub fn cell_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        let index = self.cell_index(x, y);
        debug!("x, y, index, #cells, {}, {}, {}, {}", x, y, index, self.cells.len());
        self.cells.get(index).unwrap().blocked 
        // false
    }

    pub fn get_cells(&self) -> &[Cell] {
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

    pub fn get_cell_ref(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize * self.width + x as usize]
    }

    pub fn get_mut_cell_ref(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[y as usize * self.width + x as usize]
    }

    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.index(x, y);
        self.cells[index] = cell;
    }

    fn index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    pub fn get_rooms(&self) -> &[Rect] {
        &self.rooms
    }

    pub fn set_rooms(&mut self, rooms: Vec<Rect>) {
        self.rooms = rooms;
    }
}

pub struct MapBuilder {
    width: usize,
    height: usize,
    map: Map,
}

impl MapBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            map: Map::new(width, height)
        }
    }

    pub fn create_room(mut self, room: &Rect) -> Self {
        let cells = &mut self.map.cells;

        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                let mut cell = &mut cells[y as usize * self.width + x as usize];
                MapBuilder::dig_cell(&mut cell);
            }
        }

        self
    }

    pub fn create_h_tunnel(mut self, x1: i32, x2: i32, y: i32) -> Self {
        let cells = &mut self.map.cells;

        for x in x1.min(x2)..(x1.max(x2) + 1) {
            let mut cell = &mut cells[y as usize * self.width + x as usize];
            MapBuilder::dig_cell(&mut cell);
        }

        self
    }

    pub fn create_v_tunnel(mut self, y1: i32, y2: i32, x: i32) -> Self {
        let cells = &mut self.map.cells;

        for y in y1.min(y2)..(y1.max(y2) + 1) {
            let mut cell = &mut cells[y as usize * self.width + x as usize];
            MapBuilder::dig_cell(&mut cell);
        }

        self
    }

    fn dig_cell(cell: &mut Cell) {
        cell.glyph = '.';
        cell.blocked = false;
        cell.block_sight = false;
    }

    pub fn build(self) -> Map {
        self.map
    }
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

pub const Wall: Cell = Cell { glyph: '#', blocked: true, block_sight: true };

struct Arena<T: std::fmt::Debug> {
    pub nodes: Vec<Node<T>>
}

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize 
}

#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child:Option<NodeId>,
    pub data: T
}

impl<T: std::fmt::Debug> Arena<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(node_id.index)
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: data
        });

        NodeId { index: next_index }
    }

    pub fn get_all_leaf_nodes(&self, node_id: NodeId) -> Vec<NodeId> {
        let mut leaf_node_ids = vec![];

        let node = &self.nodes[node_id.index];

        if node.first_child.is_none() {
            leaf_node_ids.push(node_id);
        } else {
            if let Some(left_node_id) = node.first_child {
                leaf_node_ids.append(&mut self.get_all_leaf_nodes(left_node_id))
            } else if let Some(right_node_id) = node.last_child {
                leaf_node_ids.append(&mut self.get_all_leaf_nodes(right_node_id))
            }
        }

        return leaf_node_ids
    }
}

fn split_dungeon(node_id: NodeId, arena: &mut Arena<Rect>) {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let max_room_count = 2;

    if node_id.index > max_room_count {
        return;
    }

    let horizontal = false;

    let node = &mut arena.nodes[node_id.index];

    let (left, right) = if horizontal {
        let y = rng.gen_range(node.data.y1, node.data.y2);

        let top_room = Rect::new(node.data.x1, node.data.y1, node.data.width(), y - node.data.y1);
        let bottom_room = Rect::new(node.data.x1, y, node.data.width(), node.data.y2 - y);

        (top_room, bottom_room)
    } else {
        let x = rng.gen_range(node.data.x1, node.data.x2);

        let left_room = Rect::new(node.data.x1, node.data.y1, x - node.data.x1, node.data.height());
        let right_room = Rect::new(x, node.data.y1, node.data.x2 - x, node.data.height());

        (left_room, right_room)
    };

    let left_node_id = arena.new_node(left);
    let right_node_id = arena.new_node(right);

    if let Some(mut parent_node) = arena.nodes.get_mut(node_id.index) {
        parent_node.first_child = Some(left_node_id);
        parent_node.last_child = Some(right_node_id);
    }

    if let Some(mut left_node) = arena.nodes.get_mut(left_node_id.index) {
        left_node.parent = Some(node_id);
    }

    if let Some(mut right_node) = arena.nodes.get_mut(right_node_id.index) {
        right_node.parent = Some(node_id);
    }

    split_dungeon(left_node_id, arena);
    split_dungeon(right_node_id, arena);
}

pub fn bsp_map_generator(width: usize, height: usize) -> Map {
    let map = Map::new(width, height);

    let root_rect = Rect::new(0, 0, width as i32, height as i32);

    let mut bsp = Arena::new();
    let root_node_id = bsp.new_node(root_rect);

    split_dungeon(root_node_id, &mut bsp);
    // Recursively split room
    // Choose a random position
    // Split the dungeon into two sub dungeons

    debug!("{:?}", bsp.nodes);
    let leaves = bsp.get_all_leaf_nodes(NodeId { index: 0 });
    debug!("Leaves {:?}", leaves);

    // Shrink Resize each room into random sizes
    // Build corriders through all the leafs of the tree
    // Connecting each leaf to its sister

    map
}

pub fn ca_map_gen(width: usize, height: usize) -> Map {
    let mut map = Map::new(width, height);

    let mut rng = rand::thread_rng();

    map.fill(Cell { glyph: '.', blocked: false, block_sight: false });

    // Fil 45% of the map
    let original_fill_amount = 45;

    for _ in 0..original_fill_amount {
        let x = rng.gen_range(0, map.width() as i32);
        let y = rng.gen_range(0, map.height() as i32);

        map.set_cell(x, y, Wall);
    }

    for _ in 0..5 {
        for y in 0..map.height() {
            for x in 0..map.width() {
                let mut wall_neighbors = 0;

                if x > 0 {

                }
            }
        }
    }

    map
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