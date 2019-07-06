use entity::{Entities, Entity};
use map::Map;

pub struct World {
    entities: Entities,
    current_map: Option<Map>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
            current_map: None
        }
    }

    pub fn is_tile_occupied(&self, x: i32, y: i32) -> Option<&dyn Entity> {
        None
    }

    pub fn register_entity(&mut self, id: &str, mut entity: Box<dyn Entity>) {
        // entity.set_world(self);
        self.entities.insert(id.to_string(), entity);
    }

    fn is_cell_blocked(&self, x: i32, y: i32) -> bool {
        if let Some(map) = self.current_map.as_ref() {
            map.is_blocked(x, y) 
        } else {
            false
        }
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn get_entity(&self, entity_id: &str) -> Option<&Box<Entity>> {
        self.entities.get(entity_id)
    }

    pub fn get_current_map(&self) -> Option<&Map> {
        self.current_map.as_ref()
    }

    pub fn set_map(&mut self, map: Map) {
        self.current_map = Some(map);
    }
}