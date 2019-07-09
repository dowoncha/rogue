// use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::cell::RefCell;

use entities::Entity;
use console::Color;

pub trait Component {
    fn name(&self) -> &'static str;
}

pub struct ComponentManager {
    components: HashMap<String, Vec<(String, Box<dyn Component>)>>
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn create_component(&mut self, entity_id: &str, component: Box<dyn Component>) {
        let component_name = component.name().to_string();
        let mut component_list = self.components.get_mut(&component_name);

        match component_list {
            Some(ref mut component_list) => {
                component_list.push((entity_id.to_string(), component));
            }
            None => {
                // components.insert(component_name, vec![(entity_id.to_string(), component)]);
            }
        }
        // self.components.insert(component.name().to_string(), (entity_id.to_string(), component));
    }

    pub fn get_entity_component(&self, entity_id: &str, component_name: &str) -> Option<&Box<dyn Component>> {
        self.components.get(component_name)
            .unwrap()
            .iter()
            .find(|(eid, component)| eid == entity_id)
            .map(|(eid, component)| component)
    }

    // pub fn get_component(&self,)
}

// #[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    fn name(&self) -> &'static str {
        "position"
    }
}

pub struct Velocity {
    pub dx: i32,
    pub dy: i32
}

impl Component for Velocity {
    fn name(&self) -> &'static str {
        "velocity"
    }
}

pub struct Glyph {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color
}

impl Component for Glyph {
    fn name(&self) -> &'static str {
        "glyph"
    }
}

pub struct Physics {
    pub blocks: bool
}

impl Component for Physics {
    fn name(&self) -> &'static str {
        "physics"
    }
}

pub struct Health {
    pub max_health: i32,
    pub health: i32
}

impl Component for Health {
    fn name(&self) -> &'static str {
        "health"
    }
}

pub struct Prototype {
    entity_id: Entity
}

impl Component for Prototype {
    fn name(&self) -> &'static str {
        "prototype"
    }
}

struct Player {
}