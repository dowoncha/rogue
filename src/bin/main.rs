#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

extern crate rogue;

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

use rogue::map::{simple_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

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

fn create_map_entities(map: &Map, em: &mut EntityManager) {
    // Create tile entity prototypes

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
        rogue::monsters::create_zombie(em, center.0, center.1);
    }
}

fn load_game_entities(entity_manager: &mut EntityManager) {
    let map_width = 200;
    let map_height = 200;

    // let map = create_map();
    let map = simple_map_gen(map_width, map_height);

    let player_pos = map.rooms.first().unwrap().center();

    create_map_entities(&map, entity_manager);
    create_player(entity_manager, player_pos.0, player_pos.1);
    populate_map(&map, entity_manager);
    rogue::items::spawn_potion_of_healing(entity_manager, player_pos.0, player_pos.1 + 2);
}

fn load_game_systems(system_manager: &mut SystemManager) {
    system_manager.register_system(Chronos::new());
    system_manager.register_system(rogue::TurnSystem::new());
    system_manager.register_system(RenderSystem::new());
    system_manager.register_system(InputSystem::new());
    system_manager.register_system(RandomWalkAiSystem);
    system_manager.register_system(WalkSystem);
    system_manager.register_system(CollisionSystem);
    system_manager.register_system(AttackSystem);
    system_manager.register_system(DamageSystem);
    system_manager.register_system(MoveSystem);
    system_manager.register_system(rogue::LootSystem);
    system_manager.register_system(EventLogSystem);
    system_manager.register_system(rogue::Reaper);
    system_manager.register_system(Janitor);
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    // let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    let mut entity_manager = EntityManager::new();

    load_game_entities(&mut entity_manager);

    let mut system_manager = SystemManager::new(&mut entity_manager);

    load_game_systems(&mut system_manager);

    system_manager.mount();

    system_manager.run();

    system_manager.unmount();

    drop_ncurses();

    Ok(())
}

