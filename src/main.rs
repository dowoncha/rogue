#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

#[macro_use]
extern crate rogue;
use rogue::{
    file_logger, 
    Component, 
    ComponentType, 
    Entity, 
    EntityManager, 
    drop_ncurses, 
    System, 
    InputSystem, 
    RenderSystem, 
    CollisionSystem,
    Rect,
    WalkSystem,
    DamageSystem,
    MoveSystem,
    MapBuilder,
    Map,
    Chronos, 
};

use rogue::map::{bsp_map_generator, ca_map_gen};
use rogue::event_system::{EventSystem};
use rogue::command_system::{CommandSystem};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

struct SaveResult {
    filename: String,
}

#[cfg(test)]
mod game_tests {
    use super::*;

    struct TestEntity;

    #[test]
    fn test_set_player_name() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // assert_eq!(game.get_player().name(), "gromash");

        // game.get_mut_player().set_name("tinker");

        // assert_eq!(game.get_player().name(), "tinker");
    }

    #[test]
    fn test_spawn_player() {
        // let room = Rect {
        //     x: 0,
        //     y: 0,
        //     width: 10,
        //     height: 10,
        // };

        // let mut game = Game::new();

        // game.set_map(room);

        // game.spawn_player();

        // let player = game.get_player();
        // let room = game.get_map();

        // assert!(
        //     player.x > 0 && player.x < room.width() && player.y > 0 && player.y < room.height()
        // );
    }

    #[test]
    fn test_time() {
        let mut chronos = Chronos::new();

        let entity_id = "";

        chronos.register(entity_id);

        chronos.release(entity_id);
    }

    // player movement into monster occupied tile */
    // Moving into monster's tile attacks it */
}

#[cfg(test)]
mod save_load_tests {
    use super::*;

    #[test]
    fn test_serialize_game() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // let game_save_buffer = game.serialize().unwrap();

        // assert!(validate_save(&game_save_buffer));
    }

    #[test]
    fn test_save_game() {
        let mut game = Game::new();

        // game.get_mut_player().set_name("gromash");

        // let save_result = game.save(None).unwrap();

        // let savefilename = save_result.filename;

        // let savefile_buffer = &std::fs::read(&savefilename).expect("Save file not found");

        // let savefile_str = String::from_utf8_lossy(&savefile_buffer);

        // assert_eq!(savefile_str, game.serialize().unwrap());

        // std::fs::remove_file(savefilename).unwrap();
    }

    /* test load game */
    /**  test deserialize game **/

    #[test]
    fn test_save_rect_map_to_file() {
        let filename = "test-save-rect-map-to-file.map";

        let map = Rect {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };

        let save_result = map.save(filename);

        assert!(save_result.is_ok());

        let loadedmap_buffer = std::fs::read(filename);

        assert!(loadedmap_buffer.is_ok());

        // assert!(loadedmap_buffer.unwrap() == map.get_buffer());

        // cleanup
        std::fs::remove_file(filename);

        assert!(std::fs::read(filename).is_err());
    }
}

fn create_player(em: &mut EntityManager) {
    let player = em.create_entity();

    em.add_component(player, components::Name { name: "gromash warhammer".to_string() });
    em.add_component(player, Input::new());
    em.add_component(player, Render { glyph: '@', layer: RenderLayer::Player });
    em.add_component(player, Position{ x: 1, y: 1});
    em.add_component(player, Collidable);
    em.add_component(player, components::Health { health: 100, max_health: 100 });
    // em.add_component(player, Physics);
    em.add_component(player, Walk::new());
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

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("\u{001b}[31mHelloWorld");
    let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    // Initialize ncurses
    // init_ncurses();

    let mut entity_manager = EntityManager::new();
    let mut render_system = RenderSystem::new();
    let mut input_system = InputSystem::new();
    let mut event_system = EventSystem::new();
    let mut command_system = CommandSystem::new();
    let mut move_system = MoveSystem;
    let mut collision_system = CollisionSystem;
    let mut walk_system = WalkSystem;
    let mut damage_system = DamageSystem;
    let mut reaper = rogue::Reaper;

    render_system.mount();
    input_system.mount();
    event_system.mount(&mut entity_manager);
    command_system.mount(&mut entity_manager);

    // let map = create_map();
    let map = ca_map_gen(80, 40);
    create_map_entities(&map, &mut entity_manager);
    create_player(&mut entity_manager);
    create_monster(&mut entity_manager, 5, 7);

    // let game_time = GameTime::new();

    // game.register_entity("gametime", game_time);

    // debug!("{:?}", entity_manager);

    'main: loop {
        // event_system.process(&mut entity_manager);
        input_system.process(&mut entity_manager);

        walk_system.process(&mut entity_manager);

        // physics_system.process(&mut entity_manager);

        collision_system.process(&mut entity_manager);

        damage_system.process(&mut entity_manager);

        move_system.process(&mut entity_manager);

        reaper.process(&mut entity_manager);

        render_system.process(&mut entity_manager);
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
