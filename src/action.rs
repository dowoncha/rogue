use types::{Vector2, BoxResult};
use entity::Entity;
use std::rc::Rc;

pub trait Action {
    fn execute(&self, entity: &mut dyn Entity) -> BoxResult<()>;
}

pub type Direction = Vector2<i32>;

pub struct WalkAction {
    pub direction: Direction
}

impl Action for WalkAction {
    fn execute(&self, entity: &mut dyn Entity) -> BoxResult<()> {
        let dest = Direction {
            x: entity.get_x() + self.direction.x,
            y: entity.get_y() + self.direction.y
        };

        // See if there is an actor there
        // let mut world = entity.get_world().unwrap();
        // let world = Rc::get_mut(&mut world);
        // if let Some(target) = world.unwrap().is_tile_occupied(dest.x, dest.y) {
        //     // Attack
        //     return Ok(());
        // }

        // See if its a door

        // See if we can walk there
        entity.set_x(dest.x);
        entity.set_y(dest.y);
        // performer.x = position.x;
        // performer.y = position.y;

        // See if hero stepped on anything interesting

        Ok(())
    }

    // fn move_entity(&self, entity_id: &str, dx: i32, dy: i32) {
    //     // Immutable checks
    //     {
    //         let entities = self.entities;

    //         let entity = entities.get(entity_id);

    //         if let Some(entity) = entity {
    //             let dest_x = entity.x + dx;
    //             let dest_y = entity.y + dy;

    //             // Check if cell is blocking
    //             if self.is_cell_blocked(dest_x, dest_y) {
    //                 return
    //             }

    //             // Check if destination cell is occupied
    //             let target = entities.values().find(|&entity| entity.x == dest_x && entity.y == dest_y);
    //             if let Some(target) = target {
    //                 info!("You attack the {}", "monster");
    //                 return
    //             }
    //         }
    //     }

    //     // Actual move
    //     {
    //         let mut entities = self.entities.borrow_mut();
    //         let mut entity = entities.get_mut(entity_id);

    //         if let Some(entity) = entity {
    //             entity._move(dx, dy);
    //         }
    //     }
    // }
}

pub struct AttackAction {
}

fn alternate() {

}