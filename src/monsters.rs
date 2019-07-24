use super::{Entity, EntityManager, Component, components};

pub fn create_monster(
    em: &mut EntityManager, 
    name: &str,
    x: i32, 
    y: i32,
    glyph: char,
    baseHitPoints: i32
) -> Entity {
    let monster = em.create_entity();

    em.add_component(monster, components::Name { name: name.to_string() });
    em.add_component(monster, components::Render { glyph: glyph, layer: components::RenderLayer::Player});
    em.add_component(monster, components::Position { x: x, y: y });
    em.add_component(monster, components::Health { health: baseHitPoints, max_health: baseHitPoints });
    em.add_component(monster, components::Walk { dx: 0, dy: 0 });
    em.add_component(monster, components::Collidable);

    monster
}

pub fn create_goblin(em: &mut EntityManager, x: i32, y: i32) -> Entity {
   let goblin = create_monster(
       em,
       "goblin",
       x,
       y,
       'g',
       8
   );

   goblin
}

pub fn create_zombie(em: &mut EntityManager, x: i32, y: i32) -> Entity {
    let zombie = create_monster(
        em,
        "zombie",
        x,
        y,
        'z',
        10
    );

    em.add_component(zombie, components::RandomWalkAi);

    zombie
}