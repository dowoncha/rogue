use super::{System};
use crate::entities::{EntityManager};

pub struct SystemManager {
    systems: Vec<Box<dyn System>>
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn mount(&mut self, em: &mut EntityManager) {
        for system in &mut self.systems {
            info!("Mounting {:?}", system);
            system.mount(em);
        }
    }

    pub fn register_system<S: 'static + Sized + System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn process_systems(&self, em: &mut EntityManager) {
        for system in &self.systems {
            system.process(em);
        }
    }

    pub fn unmount(&mut self, em: &mut EntityManager) {
        for system in &mut self.systems {
            system.unmount(em);
        }
    }
}

