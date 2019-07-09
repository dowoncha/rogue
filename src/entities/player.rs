use entities::{Entity};
use engine::Engine;
use components::{Position, Velocity, Glyph, Health};
use console::Color;

pub fn create_player(engine: &Engine)  {
    let entity_id = engine.create_entity();

    let position = Box::new(Position {
        x: 0,
        y: 0    
    });

    let velocity = Box::new(Velocity {
        dx: 0,
        dy: 0
    });

    let glyph = Box::new(Glyph {
        glyph: '@',
        fg: Color(0, 255, 0),
        bg: Color(0, 0, 0)
    });

    let health = Box::new(Health {
        max_health: 100,
        health: 100
    });

    engine.register_component(&entity_id, position);
    engine.register_component(&entity_id, velocity);
    engine.register_component(&entity_id, glyph);
    engine.register_component(&entity_id, health);

    // let mut player = Entity::new();
    // let first_room_center = map.get_rooms()[0].center();
    // let player_x = first_room_center.0;
    // let player_y = first_room_center.1;
    // player.set_x(player_x);
    // player.set_y(player_y);

    // player
}