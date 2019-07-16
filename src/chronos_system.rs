use std::collections::VecDeque;

use super::{EntityManager, Component, Entity, System};

/**
 * Chronos is the time keeper
 * He has a reference to all entities
 * And allocates time to entities for actions
 */
pub struct Chronos {
    time_travelers: VecDeque<(i32, String)>,
}

impl Chronos {
    pub fn new() -> Self {
        Self {
            time_travelers: VecDeque::new(),
        }
    }

    pub fn travelers_len(&self) -> usize {
        self.time_travelers.len()
    }

    pub fn register(&mut self, entity_id: &str) {
        let current_time = -100;
        self.time_travelers
            .push_back((current_time, entity_id.to_string()));
    }

    pub fn release(&mut self, entity_id: &str) {
        if let Some(index) = self
            .time_travelers
            .iter()
            .position(|(energy, traveler)| traveler == entity_id)
        {
            self.time_travelers.remove(index);
        }
    }
}

impl System for Chronos {
    fn process(&self, em: &mut EntityManager) {

    }
}