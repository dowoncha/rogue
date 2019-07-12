use super::{System, EntityManager, Component};

use command_system::{Command};
use components::Position;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn process(&mut self, em: &mut EntityManager) {
        // 1. Get all update position component commands
        let commands = em.get_command_queue();

        // 2. Loop through all position component's to see if they occupy destination
        let positions = em.get_all_components_of_type(Position::get_component_type());

        // for command in commands.iter() {
        //     match command {
        //     }
        // }

        // 3. Replace commands list
    }
}

#[test]
fn it_should_remove_update_position_commands_if_blocked() {

}