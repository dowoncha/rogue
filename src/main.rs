#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

use ncurses as nc;

#[macro_use]
extern crate rogue;
use rogue::{
    file_logger, 
    EntityManager, 
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
    EventLogSystem,
    Map,
    Janitor
};

use rogue::map::{simple_map_gen, ca_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

use std::env;

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
}

fn create_monster(em: &mut EntityManager, x: i32, y: i32) {
    let monster = em.create_entity();

    em.add_component(monster, components::Name { name: "goblin".to_string() });
    em.add_component(monster, Render { glyph: 'G', layer: RenderLayer::Player});
    em.add_component(monster, Position { x: x, y: y });
    em.add_component(monster, components::Health { health: 10, max_health: 10 });
    em.add_component(monster, Collidable);
}

fn create_map() -> Map {
    use rand::{Rng};

    let mut rng = rand::thread_rng();

    let map_width = 200;
    let map_height = 200;

    // Generate rooms

    let min_room_size = 5;
    let max_room_size = 20;

    let max_room_count = 10;

    let mut rooms: Vec<Rect> = vec![];

    for i in 0..max_room_count {
        let new_room = Rect::new(
            rng.gen_range(0, map_width - max_room_size),
            rng.gen_range(0, map_height - max_room_size),
            rng.gen_range(min_room_size, max_room_size),
            rng.gen_range(min_room_size, max_room_size)
        );

        let intersected = rooms.iter().any(|room| room.intersect(&new_room));

        if !intersected {
            rooms.push(new_room);
        }
    }

    let mut map_builder = MapBuilder::new(map_width as usize, map_height as usize);

    for room in rooms {
        map_builder = map_builder.create_room(&room);
    }

    map_builder.build()
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
        create_monster(em, center.0, center.1);
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    let mut entity_manager = EntityManager::new();
    let mut render_system = RenderSystem::new();

    let mut input_system = InputSystem::new();
    let move_system = MoveSystem;
    let collision_system = CollisionSystem;
    let walk_system = WalkSystem;
    let attack_system = AttackSystem;
    let damage_system = DamageSystem;
    let event_log_system = EventLogSystem;
    let reaper = rogue::Reaper;
    let janitor = Janitor;

    render_system.mount();
    input_system.mount();

    // let map = create_map();
    let map = simple_map_gen(200, 200);

    let player_pos = map.rooms.first().unwrap().center();

    create_map_entities(&map, &mut entity_manager);
    create_player(&mut entity_manager, player_pos.0, player_pos.1);
    populate_map(&map, &mut entity_manager);

    'main: loop {
        input_system.process(&mut entity_manager);

        walk_system.process(&mut entity_manager);

        collision_system.process(&mut entity_manager);

        attack_system.process(&mut entity_manager);

        damage_system.process(&mut entity_manager);

        move_system.process(&mut entity_manager);

        render_system.process(&mut entity_manager);

        reaper.process(&mut entity_manager);

        event_log_system.process(&mut entity_manager);

        janitor.process(&mut entity_manager);
    }

    // game.save(None)?;
    drop_ncurses();

    Ok(())

    // let mut game = GameClient::new();
    // game.init(
    //     args
    // ).expect("Failed to init game");

    // if let Err(error) = game.run() {
    //     error!("{}", error);
    // }

    // Ok(())
}

trait Subject {
    fn register(&mut self, observer: &dyn Observer);
    fn unregister(&mut self, observer: &dyn Observer);
    fn observers(&self) -> &[&dyn Observer];
    fn notify(&self, event: String) {
        for o in self.observers() {
            o.update(event.clone());
        }
    }
}

trait Observer {
    fn update(&self, event: String);
}
