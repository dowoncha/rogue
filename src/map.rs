use rand::{thread_rng, Rng};

use crate::types::{Rect, Dimension};

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
    pub rooms: Vec<Rect>,
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

        self.map.rooms.push(*room);

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

pub fn simple_map_gen(width: usize, height: usize) -> Map {
    let mut map = MapBuilder::new(width, height);

    let mut rooms = vec![];

    let min_room_size = 5;
    let max_room_size = 30;
    let max_room_count = 30;

    let mut rng = thread_rng();

    for _ in 0..max_room_count {
        let room_width = rng.gen_range(min_room_size, max_room_size);
        let room_height = rng.gen_range(min_room_size, max_room_size);

        let x = rng.gen_range(0, width - room_width - 1);
        let y = rng.gen_range(0, height - room_height - 1);

        let new_room = Rect::new(x as i32, y as i32, room_width as i32, room_height as i32);

        let failed = rooms.iter().any(|room| new_room.intersect(room));

        if !failed {
            // No intersections, valid room
            // Get previous room center
            if rooms.len() > 0 {
                let prev_room = rooms.last().expect("No rooms found");
                let new_center = new_room.center();
                let center = prev_room.center();

                // coinflip horizontal or vertical
                if rng.gen::<bool>() {
                    map = map.create_h_tunnel(center.0, new_center.0, center.1);
                    map = map.create_v_tunnel(center.1, new_center.1, center.0);
                } else {
                    map = map.create_v_tunnel(center.1, new_center.1, center.0);
                    map = map.create_h_tunnel(center.0, new_center.0, new_center.1);
                } 
            }

            rooms.push(new_room);
        }
    }

    for room in rooms {
        map = map.create_room(&room);
    }

    map = map.create_h_tunnel(25, 55, 23);

    map.build()
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
}
