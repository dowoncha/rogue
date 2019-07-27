#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

extern crate rlua;

use rlua::{Lua, Table, RegistryKey};

extern crate rogue;

use rogue::{
    Entity,
    Component,
    file_logger, 
    EntityManager, 
    Rect,
    MapBuilder,
    Map,
};

use rogue::systems::*;

use rogue::map::{simple_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Instant, Duration};

fn create_map_entities(map: &Map, em: &mut EntityManager) {
    // Create tile entity prototypes

    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = em.create_entity();

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
    // let (goblin, _) = em.get_entity_by_name("Goblin").unwrap();

    // Create a monster at the center of each room
    for room in &map.rooms {
        let center = room.center();
        rogue::monsters::create_zombie(em, center.0, center.1);

        // let g_entity = em.create_entity();

        // em.add_component(g_entity, components::Prototype { prototype: goblin });
        // em.add_component(g_entity, components::Position { x: center.0 + 1, y: center.1 + 1 });

        // em.find_entity(components::Metaname("Goblin"));
    }
}

fn setup_register_entity(lua: &Lua) -> Arc<Mutex<HashMap<String, RegistryKey>>> {
    let entities = Arc::new(Mutex::new(HashMap::new()));

    lua.context(|lua_ctx| {

        let register_entity = {
            let entities = entities.clone();

            let register_entity = lua_ctx.create_function(move |ctx, (name, table): (String, Table)| {
                let key = ctx.create_registry_value(table)
                    .expect("should have inserted in registry");

                entities.lock().unwrap().insert(name, key);

                Ok(())
            }).unwrap();

            register_entity
        };

        lua_ctx.globals().set("register_entity", register_entity).unwrap();
    });

    entities
}

struct ScriptManager {
    lua: Lua,
    entities: Arc<Mutex<HashMap<String, RegistryKey>>>
}

impl ScriptManager {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            entities: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn init(&mut self) {
        self.load_lua_globals();

        self.load_game_assets();
    }

    fn load_lua_globals(&mut self) {
        self.entities = setup_register_entity(&self.lua);
    }

    pub fn load_game_assets(&self) {
        self.load_asset("assets/goblin.lua");
    }

    pub fn load_asset(
        &self, 
        asset_name: &str
    ) {
        use std::io::Read;

        let mut buffer = String::new();
        let mut asset_file = std::fs::File::open(asset_name).unwrap();

        asset_file.read_to_string(&mut buffer).unwrap();

        self.lua.context(|lua_ctx| {
            lua_ctx.load(&buffer)
                .exec()
                .unwrap();
        });
    }
}

fn load_entity(entity_manager: &mut EntityManager, name: &str, table: Table) {
    let entity = entity_manager.create_entity();

    entity_manager.set_entity_name(entity, name);

    let glyph: rlua::Result<String> = table.get("glyph");

    if let Ok(glyph) = glyph {
        let glyph = glyph.chars().next().unwrap();
        entity_manager.add_component(entity, components::Render { glyph: glyph, layer: components::RenderLayer::Player });
    }

    let max_health: rlua::Result<i32> = table.get("max_health");

    if let Ok(max_health) = max_health {
        entity_manager.add_component(entity, components::Health { health: max_health, max_health: max_health });
    }

    let collidable: rlua::Result<bool> = table.get("collidable");

    if let Ok(collidable) = collidable {

    }
}

struct Game {
    script_manager: ScriptManager,
    entity_manager: EntityManager,
    system_manager: SystemManager,
    render_system: RenderSystem,
    input_system: InputSystem,
    running: bool
}

impl Game {
    pub fn new() -> Self {
        Self {
            input_system: InputSystem::new(),
            render_system: RenderSystem::new(),
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
            script_manager: ScriptManager::new(),
            running: false
        }
    }

    pub fn init(&mut self) {
        self.render_system.mount(&mut self.entity_manager);

        self.input_system.mount(&mut self.entity_manager);

        self.system_manager.mount(&mut self.entity_manager);

        self.script_manager.init();

        self.script_manager.load_game_assets();

        self.load_game_systems();

        self.load_game_entities();

        self.running = true;
    }

    fn load_game_systems(&mut self) {
        let system_manager = &mut self.system_manager;
        system_manager.register_system(Chronos::new());
        system_manager.register_system(TurnSystem::new());
        system_manager.register_system(RandomWalkAiSystem);
        system_manager.register_system(WalkSystem);
        system_manager.register_system(CollisionSystem);
        system_manager.register_system(AttackSystem);
        system_manager.register_system(DamageSystem);
        system_manager.register_system(MoveSystem);
        system_manager.register_system(LootSystem);
        system_manager.register_system(EventLogSystem);
        system_manager.register_system(Reaper);
        system_manager.register_system(Janitor);
    }

    fn load_game_entities(
        &mut self, 
    ) {
        let map_width = 100;
        let map_height = 100;

        // let map = create_map();
        let map = simple_map_gen(map_width, map_height);

        let player_pos = map.rooms.first().unwrap().center();

        create_map_entities(&map, &mut self.entity_manager);
        let player_components = self.create_player("gromash", player_pos.0, player_pos.1);

        let player = self.entity_manager.create_entity();

        for component in player_components {
            self.entity_manager.add_boxed_component(player, component);
        }

        populate_map(&map, &mut self.entity_manager);
    }

    fn create_player(
        &self,
        name: &str,
        x: i32, 
        y: i32
    ) -> Vec<Box<dyn Component>> {
        vec![
            Box::new(components::Player),
            Box::new(components::Name { name: name.to_string() }),
            Box::new(Input::new()),
            Box::new(Render { glyph: '@', layer: RenderLayer::Player }),
            Box::new(Position{ x: x, y: y}),
            Box::new(Collidable),
            Box::new(components::Health { health: 100, max_health: 100 }),
            Box::new(Walk::new()),
            Box::new(components::Log::new()),
            Box::new(components::Energy { amount: 0 }),
            Box::new(components::Speed { amount: 10 })
        ]
    }

    pub fn run(&mut self) {
        let mut last_time = Instant::now();

        while self.is_running() {
            let current = Instant::now();

            let elapsed: Duration = current.duration_since(last_time);

            self.handle_input();

            // Send inputs to engine

            self.update(elapsed);

            // Get a copy of the world 

            self.render();

            last_time = current;
        }

        self.cleanup();
    }

    fn handle_input(&mut self) {
        self.input_system.process(&mut self.entity_manager);

        match self.input_system.get_last_input() { 
            Some(113) => {
                self.quit();
            },
            _ => {}
        }
    }
    
    fn update(&mut self, elapsed: Duration) {
        self.system_manager.process_systems(&mut self.entity_manager);
    }

    fn render(&mut self) {
        self.render_system.process(&mut self.entity_manager);
    }

    fn cleanup(&mut self) {
        self.system_manager.unmount(&mut self.entity_manager);

        self.render_system.unmount();
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

struct Engine {
    entity_manager: EntityManager,
    system_manager: SystemManager,
    connections: Vec<std::sync::mpsc::Receiver<String>>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
            connections: Vec::new()
        }
    }

    pub fn init(&mut self) {

    }

    pub fn run(&mut self) {
        loop {
            self.system_manager.process_systems(&mut self.entity_manager);
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    file_logger::init()
        .expect("Failed to init file logger");

    let mut game = Game::new();

    game.init();

    game.run();

    Ok(())
}

