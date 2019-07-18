#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

use ncurses as nc;

#[macro_use]
extern crate rogue;

extern crate rlua; 

use rlua::{Lua};

use rogue::{
    Entity,
    Component,
    file_logger, 
    EntityManager, 
    SystemManager,
    drop_ncurses, 
    System, 
    InputSystem, 
    RenderSystem, 
    CollisionSystem,
    AttackSystem,
    Rect,
    WalkSystem,
    DamageSystem,
    MoveSystem,
    MapBuilder,
    Chronos,
    EventLogSystem,
    RandomWalkAiSystem,
    Map,
    Janitor
};

use rogue::map::{simple_map_gen, ca_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

use std::env;
use std::collections::HashMap;

fn create_player(em: &mut EntityManager, x: i32, y: i32) {
    let player = em.create_entity();

    em.add_component(player, components::Name { name: "gromash warhammer".to_string() });
    em.add_component(player, components::Player);
    em.add_component(player, Input::new());
    em.add_component(player, Render { glyph: '@', layer: RenderLayer::Player });
    em.add_component(player, Position{ x: x, y: y});
    em.add_component(player, Collidable);
    em.add_component(player, components::Health { health: 100, max_health: 100 });
    em.add_component(player, Walk::new());
    em.add_component(player, components::Log::new());
    em.add_component(player, components::Energy { amount: 0 });
    em.add_component(player, components::Speed { amount: 50 });
}

fn create_monster(
    em: &mut EntityManager, 
    name: &str,
    x: i32, 
    y: i32,
    glyph: char,
    baseHitPoints: i32
) -> Entity {
    let monster = em.create_entity();

    em.add_component(monster, components::Name { name: name.to_string() });
    em.add_component(monster, Render { glyph: glyph, layer: RenderLayer::Player});
    em.add_component(monster, Position { x: x, y: y });
    em.add_component(monster, components::Health { health: baseHitPoints, max_health: baseHitPoints });
    em.add_component(monster, components::Walk { dx: 0, dy: 0 });
    em.add_component(monster, Collidable);

    monster
}

fn create_goblin(em: &mut EntityManager, x: i32, y: i32) -> Entity {
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

fn create_zombie(em: &mut EntityManager, x: i32, y: i32) -> Entity {
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

fn create_map_entities(map: &Map, em: &mut EntityManager) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = em.create_entity();
            let index = (y * map.width() + x) as usize;

            let cell = map.get_cell_ref(x as i32, y as i32);

            let glyph = cell.glyph;

            em.add_component(tile, Render { glyph: glyph, layer: RenderLayer::Map });
            em.add_component(tile, Position{ x: x as i32, y: y as i32});

            if glyph == '-' || glyph == '|' || glyph == '#' {
                em.add_component(tile, Collidable);
            }
        }
    }
}

fn populate_map(map: &Map, em: &mut EntityManager) {
    // Create a monster at the center of each room
    for room in &map.rooms {
        let center = room.center();
        create_zombie(em, center.0, center.1);
    }
}

fn load_breeds(filename: &str, em: &mut EntityManager) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::prelude::*;

    info!("Loading {}", filename);
    let lua = Lua::new();

    let mut file = std::fs::File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let mut breeds = None;

    let _: rlua::Result<()> = lua.context(|ctx| {
        ctx.load(&buffer)
            .set_name("breeds")
            .expect("Failed to set name")
            .exec()
            .expect("Failed to exec chunk");

        let globals = ctx.globals();

        breeds = globals.get::<_, HashMap<String, HashMap<String, String>>>("breeds").ok();

        Ok(())
    });

    debug!("{:?}", breeds);

    for (breed, attributes) in breeds.unwrap().iter() {
        let entity_template = em.create_entity();

        em.add_component(entity_template, components::Name { name: breed.to_string() });

        for (attribute, value) in attributes.iter() {
            match attribute.as_str() {
                "glyph" => {
                    let glyph = value.chars().next().unwrap();
                    // em.add_component(entity_template, components::Render { glyph: glyph, layer: RenderLayer::Player });
                }
                _ => { debug!("Unimplemented attribute {}", attribute); }
            }
        }
    }

    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    // let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    let mut entity_manager = EntityManager::new();
    let map_width = 200;
    let map_height = 200;

    // let map = create_map();
    let map = simple_map_gen(map_width, map_height);

    let player_pos = map.rooms.first().unwrap().center();

    // load_breeds("assets/breeds.lua", &mut entity_manager)?;

    info!("Assets loaded");

    create_map_entities(&map, &mut entity_manager);
    create_player(&mut entity_manager, player_pos.0, player_pos.1);
    populate_map(&map, &mut entity_manager);

    create_zombie(&mut entity_manager, player_pos.0 + 2, player_pos.1);

    let mut system_manager = SystemManager::new(&mut entity_manager);

    system_manager.register_system(Chronos::new());
    system_manager.register_system(RenderSystem::new());
    system_manager.register_system(InputSystem::new());
    // register TimeSystem
    // register_ai_system();
    // register_gameplay_system();
    // register_render_system();
    system_manager.register_system(RandomWalkAiSystem);
    system_manager.register_system(WalkSystem);
    system_manager.register_system(CollisionSystem);
    system_manager.register_system(AttackSystem);
    system_manager.register_system(DamageSystem);
    system_manager.register_system(MoveSystem);
    system_manager.register_system(EventLogSystem);
    system_manager.register_system(rogue::Reaper);
    system_manager.register_system(Janitor);

    system_manager.mount();

    system_manager.run();

    system_manager.unmount();

    drop_ncurses();

    Ok(())
}

