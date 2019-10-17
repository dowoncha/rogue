use super::{System};
use entities::{EntityManager};
use components::{Component, self};
use rand::{Rng, thread_rng};


#[derive(Debug)]
pub struct RandomWalkAiSystem;

impl System for RandomWalkAiSystem {
    fn process(&self, em: &mut EntityManager) {
        debug!("Processing random walk ai system");
        let mut rng = thread_rng();

        let entities = em.get_entities_with_components(components::RandomWalkAi::get_component_type());

        for entity in entities {
            if let Some(walk) = get_component!(mut, em, entity, components::Walk) {
                walk.dx = rng.gen_range(-1, 2);
                walk.dy = rng.gen_range(-1, 2);
            }
        }
    }
}