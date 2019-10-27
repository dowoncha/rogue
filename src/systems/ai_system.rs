use super::System;
use crate::entities::{EntityManager};

#[derive(Debug)]
pub struct AiSystem;

impl System for AiSystem {
    fn process(&self, em: &mut EntityManager) {
        // Whose turn is it?

        // Does the entity have a script

        // run the script
    }
}